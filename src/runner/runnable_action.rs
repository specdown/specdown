use crate::results::test_result::TestResult;
use crate::runner::executor::Executor;
use crate::runner::state::State;
use crate::runner::{error, file, script, verify, Error};
use crate::types::{Action, CreateFileAction, ScriptAction, VerifyAction};

pub fn from_action(action: &Action) -> &dyn RunnableAction {
    match action {
        Action::Script(a) => a,
        Action::Verify(a) => a,
        Action::CreateFile(a) => a,
    }
}

pub trait RunnableAction {
    fn run(&self, state: &State, executor: &dyn Executor) -> Result<TestResult, error::Error>;
}

impl RunnableAction for ScriptAction {
    fn run(&self, _state: &State, executor: &dyn Executor) -> Result<TestResult, Error> {
        script::run(
            &self.script_name,
            &self.script_code,
            &self.expected_exit_code,
            executor,
        )
    }
}

impl RunnableAction for VerifyAction {
    fn run(&self, state: &State, _executor: &dyn Executor) -> Result<TestResult, Error> {
        verify::run(&self.source, &self.expected_value, state)
    }
}

impl RunnableAction for CreateFileAction {
    fn run(&self, _state: &State, _executor: &dyn Executor) -> Result<TestResult, Error> {
        Ok(file::run(&self.file_path, &self.file_content))
    }
}
