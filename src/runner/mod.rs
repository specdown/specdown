mod state;

use crate::results::test_result::TestResult;
use crate::runner::state::State;
use crate::types::Action;

mod error;
mod executor;
mod file;
mod script;
mod verify;

use executor::{Executor, Shell};

pub use error::Error;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub enum RunEvent {
    SpecFileStarted(PathBuf),
    TestCompleted(TestResult),
    SpecFileCompleted { success: bool },
}

pub fn run_actions(
    spec_file: &Path,
    actions: &[Action],
    shell_command: &str,
) -> Result<Vec<RunEvent>, Error> {
    let mut events = vec![RunEvent::SpecFileStarted(spec_file.to_path_buf())];
    let mut state = State::new();
    let executor = Shell::new(shell_command)?;

    for action in actions {
        let result = run_action(action, &state, &executor)?;
        state.add_result(&result);
        events.push(RunEvent::TestCompleted(result))
    }

    events.push(RunEvent::SpecFileCompleted {
        success: state.is_success(),
    });

    Ok(events)
}

fn run_action(
    action: &Action,
    state: &State,
    executor: &dyn Executor,
) -> Result<TestResult, error::Error> {
    match action {
        Action::Script {
            script_name,
            script_code,
            expected_exit_code,
        } => script::run(script_name, script_code, expected_exit_code, executor),
        Action::Verify {
            source,
            expected_value,
        } => verify::run(source, expected_value, state),
        Action::CreateFile {
            file_path,
            file_content,
        } => Ok(file::run(file_path, file_content)),
    }
}
