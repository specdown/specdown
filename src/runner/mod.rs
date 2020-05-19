mod state;

use std::process::Command;

use crate::results::printer::Printer;
use crate::results::test_result::TestResult;
use crate::runner::state::State;
use crate::types::{Action, ScriptCode, ScriptName, Source, VerifyValue};

enum RunnerError {
    CommandFailed,
}

pub fn run_actions(actions: &[Action], printer: &dyn Printer) {
    let mut state = State::new();

    for action in actions {
        match run_action(action, &mut state) {
            Ok(result) => printer.print(&result),
            Err(_err) => break,
        }
    }

    if !state.is_success() {
        std::process::exit(1);
    }
}

fn run_action(action: &Action, state: &mut State) -> Result<TestResult, RunnerError> {
    match action {
        Action::Script(name, code) => run_script(name, code, state),
        Action::Verify(source, value) => run_verify(source, value, state),
    }
}

fn run_script(
    name: &ScriptName,
    code: &ScriptCode,
    state: &mut State,
) -> Result<TestResult, RunnerError> {
    let ScriptName(name_string) = name;
    let ScriptCode(code_string) = code;

    let command_result = Command::new("sh").arg("-c").arg(code_string).output();

    match command_result {
        Ok(output) => {
            let output_string = String::from_utf8_lossy(&output.stdout).to_string();
            let result = TestResult::ScriptResult {
                name: name_string.to_string(),
                exit_code: 0,
                script: code_string.to_string(),
                output: output_string,
                stdout: "FIXME stderr".to_string(),
                stderr: "FIXME stderr1".to_string(),
                success: true,
            };
            state.add_result(&result);
            Ok(result)
        }
        Err(_err) => Err(RunnerError::CommandFailed),
    }
}

fn run_verify(
    source: &Source,
    value: &VerifyValue,
    state: &mut State,
) -> Result<TestResult, RunnerError> {
    let Source {
        name: ScriptName(script_name),
        stream: _stream,
    } = source;
    let VerifyValue(value_string) = value;

    let got = state.get_script_output(script_name).expect("failed");

    let result = TestResult::VerifyResult {
        script_name: script_name.to_string(),
        stream: "FIXME output".to_string(),
        expected: value_string.to_string(),
        got: got.to_string(),
        success: value_string == got,
    };

    state.add_result(&result);

    Ok(result)
}
