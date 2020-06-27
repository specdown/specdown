use crate::results::test_result::TestResult;
use crate::types::{ExitCode, ScriptCode, ScriptName};

use super::error::Error;
use super::executor::{Executor, Output};

pub fn run(
    name: &ScriptName,
    code: &ScriptCode,
    expected_exit_code: &Option<ExitCode>,
    executor: &dyn Executor,
) -> Result<TestResult, Error> {
    let ScriptCode(code_string) = code;
    let ScriptName(name_string) = name;

    executor.execute(code).map(
        |Output {
             stdout,
             stderr,
             exit_code,
         }| {
            let expected_exit = expected_exit_code.clone().map(|ExitCode(code)| code);

            TestResult::Script {
                name: name_string.to_string(),
                exit_code,
                expected_exit_code: expected_exit,
                script: code_string.to_string(),
                stdout,
                stderr,
                success: expected_exit == None || expected_exit == exit_code,
            }
        },
    )
}
