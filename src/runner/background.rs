use std::process::Child;

use crate::results::{ActionResult, BackgroundStartResult};
use crate::types::BackgroundAction;

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
    let _ = bg.child.kill();
    let _ = bg.child.wait();
    ActionResult::BackgroundStop(crate::results::BackgroundStopResult {
        script_name: bg.script_name,
    })
}
