mod state;

use crate::results::printer::PrintItem;
use crate::results::test_result::TestResult;
use crate::runner::state::State;
use crate::types::Action;

mod error;
mod executor;
mod file;
mod script;
mod verify;

use executor::{Executor, Shell};

use crate::exit_codes::ExitCode;
pub use error::Error;
pub use state::Summary;

pub fn run_actions(
    actions: &[Action],
    shell_command: &str,
) -> Result<(ExitCode, Vec<PrintItem>), Error> {
    let mut state = State::new();
    let executor = Shell::new(shell_command)?;
    let mut print_items = Vec::new();

    for action in actions {
        let result = run_action(action, &state, &executor)?;
        state.add_result(&result);
        print_items.push(PrintItem::TestResult(result));
    }

    print_items.push(PrintItem::SpecFileSummary(state.summary()));

    let exit_code = if state.is_success() {
        ExitCode::Success
    } else {
        ExitCode::TestFailed
    };

    Ok((exit_code, print_items))
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
