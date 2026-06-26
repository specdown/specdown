pub use error::Error;
pub use executor::Executor;
pub use run_event::RunEvent;
pub use runnable_action::to_runnable;
pub use state::State;

use crate::types::Action;

mod background;
mod error;
mod executor;
mod file;
mod run_event;
mod runnable_action;
mod script;
pub mod shell_executor;
mod state;
mod verify;

pub struct Runner<'a> {
    executor: &'a dyn Executor,
    state: &'a mut State,
    background_processes: Vec<background::BackgroundProcess>,
}

impl<'a> Runner<'a> {
    pub fn create(executor: &'a dyn Executor, state: &'a mut State) -> Self {
        Runner {
            executor,
            state,
            background_processes: Vec::new(),
        }
    }

    pub fn run(&mut self, actions: &[Action]) -> Vec<RunEvent> {
        let mut events: Vec<RunEvent> = actions
            .iter()
            .map(|action| self.run_action(action))
            .collect();

        // Stop all background processes
        for bg in self.background_processes.drain(..) {
            let result = background::stop(bg);
            self.state.add_result(&result);
            events.push(RunEvent::TestCompleted(result));
        }

        events
    }

    fn run_action(&mut self, action: &Action) -> RunEvent {
        match action {
            Action::Background(bg_action) => match background::start(bg_action, self.executor) {
                Ok((result, bg_process)) => {
                    self.state.add_result(&result);
                    self.background_processes.push(bg_process);
                    RunEvent::TestCompleted(result)
                }
                Err(error) => RunEvent::ErrorOccurred(error),
            },
            _ => to_runnable(action)
                .run(self.state, self.executor)
                .map(|result| {
                    self.state.add_result(&result);
                    RunEvent::TestCompleted(result)
                })
                .or_else::<Error, _>(|error| Ok(RunEvent::ErrorOccurred(error)))
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::executor::Output;
    use crate::types::{
        CreateFileAction, FileContent, FilePath, OutputExpectation, ScriptAction, ScriptCode,
        ScriptName,
    };
    use std::cell::RefCell;

    struct MockExecutor {
        output: RefCell<Option<Result<Output, Error>>>,
    }

    impl MockExecutor {
        fn with_success(exit_code: Option<i32>, stdout: &str, stderr: &str) -> Self {
            Self {
                output: RefCell::new(Some(Ok(Output {
                    stdout: stdout.to_string(),
                    stderr: stderr.to_string(),
                    exit_code,
                }))),
            }
        }

        fn with_error(error: Error) -> Self {
            Self {
                output: RefCell::new(Some(Err(error))),
            }
        }
    }

    impl Executor for MockExecutor {
        fn execute(&self, _script: &ScriptCode) -> Result<Output, Error> {
            self.output
                .borrow_mut()
                .take()
                .expect("mock executor called more times than expected")
        }
    }

    #[test]
    fn run_returns_events_for_each_action() {
        let mock = MockExecutor::with_success(Some(0), "hello", "");
        let mut state = State::new();
        let mut runner = Runner::create(&mock, &mut state);

        let actions = vec![
            Action::Script(ScriptAction {
                script_name: Some(ScriptName("script1".to_string())),
                script_code: ScriptCode("echo hello".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            }),
            Action::CreateFile(CreateFileAction {
                file_path: FilePath("test.txt".to_string()),
                file_content: FileContent("content".to_string()),
            }),
        ];

        let events = runner.run(&actions);

        // The mutant replaces run() with empty Vec, so we must assert:
        // 1. The Vec is non-empty
        assert!(!events.is_empty(), "run() should return events for each action");

        // 2. The Vec has the same length as the input actions
        assert_eq!(events.len(), actions.len(), "run() should return one event per action");

        // 3. The events contain the correct variants
        match &events[0] {
            RunEvent::TestCompleted(result) => {
                // The script action should produce a successful Script result
                assert!(result.success(), "script action should succeed");
            }
            RunEvent::ErrorOccurred(_) => panic!("script action should not produce an error"),
            _ => panic!("unexpected event type for script action"),
        }

        match &events[1] {
            RunEvent::TestCompleted(result) => {
                // The create file action should produce a successful CreateFile result
                assert!(result.success(), "create file action should succeed");
            }
            RunEvent::ErrorOccurred(_) => panic!("create file action should not produce an error"),
            _ => panic!("unexpected event type for create file action"),
        }
    }

    #[test]
    fn run_returns_error_event_when_script_fails() {
        let mock = MockExecutor::with_error(Error::CommandFailed {
            command: "bad_cmd".to_string(),
            message: "not found".to_string(),
        });
        let mut state = State::new();
        let mut runner = Runner::create(&mock, &mut state);

        let actions = vec![Action::Script(ScriptAction {
            script_name: Some(ScriptName("fail_script".to_string())),
            script_code: ScriptCode("bad_cmd".to_string()),
            expected_exit_code: None,
            expected_output: OutputExpectation::Any,
        })];

        let events = runner.run(&actions);

        assert_eq!(events.len(), 1, "run() should return one event for one action");
        match &events[0] {
            RunEvent::ErrorOccurred(error) => {
                assert!(
                    matches!(error, Error::CommandFailed { .. }),
                    "expected CommandFailed error, got: {error:?}"
                );
            }
            RunEvent::TestCompleted(_) => panic!("expected error event, got TestCompleted"),
            _ => panic!("unexpected event type"),
        }
    }

    #[test]
    fn run_returns_empty_vec_for_empty_actions() {
        let mock = MockExecutor::with_success(Some(0), "", "");
        let mut state = State::new();
        let mut runner = Runner::create(&mock, &mut state);

        let events = runner.run(&[]);
        assert!(events.is_empty(), "run() with no actions should return empty Vec");
    }

    #[test]
    fn run_updates_state_with_results() {
        let mock = MockExecutor::with_success(Some(0), "output", "");
        let mut state = State::new();
        let mut runner = Runner::create(&mock, &mut state);

        let actions = vec![Action::Script(ScriptAction {
            script_name: Some(ScriptName("state_test".to_string())),
            script_code: ScriptCode("echo".to_string()),
            expected_exit_code: None,
            expected_output: OutputExpectation::Any,
        })];

        runner.run(&actions);

        // After running, the state should have recorded the result
        assert!(state.is_success(), "state should be success after a passing script");
    }
}
