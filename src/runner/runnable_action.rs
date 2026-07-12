use std::path::Path;

use crate::results::ActionResult;
use crate::types::{
    Action, BackgroundAction, CreateFileAction, ResponseAction, ScriptAction, VerifyAction,
};

use super::{error, file, script, verify, Error, Executor, State};

pub fn to_runnable(action: &Action) -> &dyn RunnableAction {
    match action {
        Action::Script(a) => a,
        Action::Verify(a) => a,
        Action::CreateFile(a) => a,
        Action::Background(a) => a,
        Action::Response(a) => a,
    }
}

pub trait RunnableAction {
    fn run(
        &self,
        state: &State,
        executor: &dyn Executor,
        working_dir: &Path,
    ) -> Result<ActionResult, error::Error>;
}

impl RunnableAction for ScriptAction {
    fn run(
        &self,
        _state: &State,
        executor: &dyn Executor,
        _working_dir: &Path,
    ) -> Result<ActionResult, Error> {
        script::run(self, executor)
    }
}

impl RunnableAction for VerifyAction {
    fn run(
        &self,
        state: &State,
        _executor: &dyn Executor,
        _working_dir: &Path,
    ) -> Result<ActionResult, Error> {
        verify::run(self, state)
    }
}

impl RunnableAction for CreateFileAction {
    fn run(
        &self,
        _state: &State,
        _executor: &dyn Executor,
        working_dir: &Path,
    ) -> Result<ActionResult, Error> {
        Ok(file::run(self, working_dir))
    }
}

impl RunnableAction for BackgroundAction {
    fn run(
        &self,
        _state: &State,
        _executor: &dyn Executor,
        _working_dir: &Path,
    ) -> Result<ActionResult, Error> {
        // Background actions are handled specially by the Runner,
        // not through the normal RunnableAction trait.
        // This implementation should not be reached.
        Err(Error::BackgroundNotSupported)
    }
}

impl RunnableAction for ResponseAction {
    fn run(
        &self,
        _state: &State,
        _executor: &dyn Executor,
        _working_dir: &Path,
    ) -> Result<ActionResult, Error> {
        // Response actions require the mock server, which is not part of this
        // task. The Runner handles them specially (like Background actions);
        // reaching this trait method means the mock server was never started.
        Err(Error::MockServerNotStarted)
    }
}
