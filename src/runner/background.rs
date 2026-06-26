use std::process::Child;

use crate::results::{
    ActionResult, BackgroundExitStatus, BackgroundStartResult, BackgroundStopResult,
};
use crate::types::BackgroundAction;
use crate::types::ExitCode;

use super::executor::Executor;
use super::Error;

pub struct BackgroundProcess {
    pub child: Child,
    pub script_name: Option<crate::types::ScriptName>,
}

pub fn start(
    action: &BackgroundAction,
    executor: &dyn Executor,
) -> Result<(ActionResult, BackgroundProcess), Error> {
    let BackgroundAction {
        script_name,
        script_code,
    } = action;

    let child = executor.spawn(script_code)?;

    let result = ActionResult::BackgroundStart(BackgroundStartResult {
        action: action.clone(),
    });

    Ok((
        result,
        BackgroundProcess {
            child,
            script_name: script_name.clone(),
        },
    ))
}

pub fn stop(mut bg: BackgroundProcess) -> ActionResult {
    // Check if the process has already exited on its own.
    match bg.child.try_wait() {
        Ok(Some(status)) => {
            // The process exited before we could kill it.
            let exit_code = status.code().unwrap_or(-1);
            ActionResult::BackgroundStop(BackgroundStopResult {
                script_name: bg.script_name,
                exit_status: BackgroundExitStatus::Exited(ExitCode(exit_code)),
            })
        }
        Ok(None) => {
            // The process is still running — kill it and wait for it to exit.
            let _ = bg.child.kill();
            let _ = bg.child.wait();
            ActionResult::BackgroundStop(BackgroundStopResult {
                script_name: bg.script_name,
                exit_status: BackgroundExitStatus::Killed,
            })
        }
        Err(_) => {
            // Could not determine the process status — kill it as a fallback.
            let _ = bg.child.kill();
            let _ = bg.child.wait();
            ActionResult::BackgroundStop(BackgroundStopResult {
                script_name: bg.script_name,
                exit_status: BackgroundExitStatus::Killed,
            })
        }
    }
}
