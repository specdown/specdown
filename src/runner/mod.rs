mod state;

use crate::results::printer::Printer;
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

pub fn run_actions(actions: &[Action], shell_command: &str, printer: &dyn Printer) {
    let result = run_all_actions(actions, shell_command, printer);

    match result {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(err) => {
            printer.print_error(&err);
            std::process::exit(2);
        }
    }
}

fn run_all_actions(
    actions: &[Action],
    shell_command: &str,
    printer: &dyn Printer,
) -> Result<bool, Error> {
    let mut state = State::new();
    let executor = Shell::new(shell_command)?;

    for action in actions {
        let result = run_action(action, &state, &executor)?;
        state.add_result(&result);
        printer.print_result(&result);
    }

    printer.print_summary(&state.summary());

    Ok(state.is_success())
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
        } => file::run(file_path, file_content),
    }
}
