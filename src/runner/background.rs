use crate::results::{
    ActionResult, BackgroundExitStatus, BackgroundStartResult, BackgroundStopResult,
};
use crate::types::BackgroundAction;
use crate::types::ExitCode;

use super::background_handle::BackgroundHandle;
use super::executor::Executor;
use super::Error;

pub struct BackgroundProcess {
    pub handle: Box<dyn BackgroundHandle>,
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

    let handle = executor.spawn(script_code)?;

    let result = ActionResult::BackgroundStart(BackgroundStartResult {
        action: action.clone(),
    });

    Ok((
        result,
        BackgroundProcess {
            handle,
            script_name: script_name.clone(),
        },
    ))
}

pub fn stop(mut bg: BackgroundProcess) -> ActionResult {
    // Check if the process has already exited on its own.
    match bg.handle.try_wait() {
        Some(exit_code) => {
            // The process exited before we could kill it.
            ActionResult::BackgroundStop(BackgroundStopResult {
                script_name: bg.script_name,
                exit_status: BackgroundExitStatus::Exited(ExitCode(exit_code)),
            })
        }
        None => {
            // The process is still running — kill it and wait for it to exit.
            bg.handle.kill();
            let _ = bg.handle.wait();
            ActionResult::BackgroundStop(BackgroundStopResult {
                script_name: bg.script_name,
                exit_status: BackgroundExitStatus::Killed,
            })
        }
    }
}
