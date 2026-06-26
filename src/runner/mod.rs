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
            Action::Background(bg_action) => {
                match background::start(bg_action, self.executor) {
                    Ok((result, bg_process)) => {
                        self.state.add_result(&result);
                        self.background_processes.push(bg_process);
                        RunEvent::TestCompleted(result)
                    }
                    Err(error) => RunEvent::ErrorOccurred(error),
                }
            }
            _ => {
                to_runnable(action)
                    .run(self.state, self.executor)
                    .map(|result| {
                        self.state.add_result(&result);
                        RunEvent::TestCompleted(result)
                    })
                    .or_else::<Error, _>(|error| Ok(RunEvent::ErrorOccurred(error)))
                    .unwrap()
            }
        }
    }
}
