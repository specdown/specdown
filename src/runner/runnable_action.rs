use crate::results::action_result::ActionResult;
use crate::runner::error::Error;
use crate::runner::executor::Executor;
use crate::runner::state::State;
use crate::runner::{error, file, script, verify};
use crate::types::{Action, CreateFileAction, ScriptAction, VerifyAction};

pub fn to_runnable(action: &Action) -> &dyn RunnableAction {
    match action {
        Action::Script(a) => a,
        Action::Verify(a) => a,
        Action::CreateFile(a) => a,
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
