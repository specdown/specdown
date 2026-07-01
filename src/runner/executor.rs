use crate::types::ScriptCode;

use super::background_handle::BackgroundHandle;
use super::Error;

pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

impl From<std::process::Output> for Output {
    fn from(output: std::process::Output) -> Self {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code();

        Self {
            stdout,
            stderr,
            exit_code,
        }
    }
}

pub trait Executor {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error>;

    fn spawn(&self, script: &ScriptCode) -> Result<Box<dyn BackgroundHandle>, Error> {
        let _ = script;
        Err(Error::BackgroundNotSupported)
    }
}
