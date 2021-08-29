use crate::results::action_result::{ActionResult, ScriptResult};
use crate::types::{ExitCode, ScriptAction};

use super::error::Error;
use super::executor::{Executor, Output};

pub fn run(action: &ScriptAction, executor: &dyn Executor) -> Result<ActionResult, Error> {
    let ScriptAction { script_code, .. } = action;

    executor.execute(script_code).map(
        |Output {
             stdout,
             stderr,
             exit_code,
         }| {
            ActionResult::Script(ScriptResult {
                action: action.clone(),
                exit_code: exit_code.map(ExitCode),
                stdout,
                stderr,
            })
        },
    )
}
