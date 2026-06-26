use crate::results::ActionResult;
use crate::types::{Action, BackgroundAction, CreateFileAction, ScriptAction, VerifyAction};

use super::{error, file, script, verify, Error, Executor, State};

pub fn to_runnable(action: &Action) -> &dyn RunnableAction {
    match action {
        Action::Script(a) => a,
        Action::Verify(a) => a,
        Action::CreateFile(a) => a,
        Action::Background(a) => a,
    }
}

pub trait RunnableAction {
    fn run(&self, state: &State, executor: &dyn Executor) -> Result<ActionResult, error::Error>;
}

impl RunnableAction for ScriptAction {
    fn run(&self, _state: &State, executor: &dyn Executor) -> Result<ActionResult, Error> {
        script::run(self, executor)
    }
}

impl RunnableAction for VerifyAction {
    fn run(&self, state: &State, _executor: &dyn Executor) -> Result<ActionResult, Error> {
        verify::run(self, state)
    }
}

impl RunnableAction for CreateFileAction {
    fn run(&self, _state: &State, _executor: &dyn Executor) -> Result<ActionResult, Error> {
        Ok(file::run(self))
    }
}

impl RunnableAction for BackgroundAction {
    fn run(&self, _state: &State, _executor: &dyn Executor) -> Result<ActionResult, Error> {
        // Background actions are handled specially by the Runner,
        // not through the normal RunnableAction trait.
        // This implementation should not be reached.
        Err(Error::BackgroundNotSupported)
    }
}
