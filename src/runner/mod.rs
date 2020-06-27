mod state;

use crate::results::printer::Printer;
use crate::results::test_result::TestResult;
use crate::runner::state::State;
use crate::types::Action;

mod error;
mod file;
mod script;
mod verify;

pub fn run_actions(actions: &[Action], printer: &dyn Printer) {
    let mut state = State::new();

    for action in actions {
        match run_action(action, &state) {
            Ok(result) => {
                state.add_result(&result);
                printer.print(&result)
            }
            Err(_err) => break,
        }
    }

    if !state.is_success() {
        std::process::exit(1);
    }
}

fn run_action(action: &Action, state: &State) -> Result<TestResult, error::Error> {
    match action {
        Action::Script {
            script_name,
            script_code,
            expected_exit_code,
        } => script::run(script_name, script_code, expected_exit_code),
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
