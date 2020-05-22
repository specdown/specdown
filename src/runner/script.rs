use std::process::Command;

use crate::results::test_result::TestResult;
use crate::runner::state::State;
use crate::types::{ScriptCode, ScriptName};

use super::error::Error;

pub fn run(name: &ScriptName, code: &ScriptCode, state: &mut State) -> Result<TestResult, Error> {
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
        Err(_err) => Err(Error::CommandFailed),
    }
}
