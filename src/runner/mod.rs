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
pub use state::Summary;

pub fn run_actions(
    actions: &[Action],
    shell_command: &str,
) -> Result<(bool, Summary, Vec<TestResult>), Error> {
    let mut state = State::new();
    let executor = Shell::new(shell_command)?;
    let mut test_results = Vec::new();

    for action in actions {
        let result = run_action(action, &state, &executor)?;
        state.add_result(&result);
        test_results.push(result);
    }

    Ok((state.is_success(), state.summary(), test_results))
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
