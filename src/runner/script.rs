use std::process::Command;

use crate::results::test_result::TestResult;
use crate::types::{ExitCode, ScriptCode, ScriptName};

use super::error::Error;

pub fn run(
    name: &ScriptName,
    code: &ScriptCode,
    expected_exit_code: &Option<ExitCode>,
) -> Result<TestResult, Error> {
    let ScriptName(name_string) = name;
    let ScriptCode(code_string) = code;

    let command_result = Command::new("bash").arg("-c").arg(code_string).output();

    match command_result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            let exit_code = output.status.code();
            let expected_exit = expected_exit_code.clone().map(|ExitCode(code)| code);
            let result = TestResult::Script {
                name: name_string.to_string(),
                exit_code,
                expected_exit_code: expected_exit,
                script: code_string.to_string(),
                stdout,
                stderr,
                success: expected_exit == None || expected_exit == exit_code,
            };
            Ok(result)
        }
        Err(_err) => Err(Error::CommandFailed),
    }
}
