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

pub trait Executor: Send + Sync {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error>;

    fn spawn(&self, script: &ScriptCode) -> Result<Box<dyn BackgroundHandle>, Error> {
        let _ = script;
        Err(Error::BackgroundNotSupported)
    }

    /// Create a boxed clone of this executor.
    ///
    /// Used when running spec files in parallel (`--jobs > 1`): each
    /// parallel spec file gets its own executor instance via this method
    /// so that stateful executors (e.g. the container executor, which
    /// manages a Docker container) get complete isolation between
    /// concurrent spec files.
    ///
    /// The `label` argument is the spec file path (or other identifying
    /// string). The container executor incorporates a hash of this label
    /// into the container name (`specdown-{hash}-{counter}`) so that
    /// containers are uniquely identifiable per spec file.
    ///
    /// The default implementation returns a `FailedExecutor` that produces
    /// a `BackgroundNotSupported` error on first use. Executors that support
    /// parallel execution should override this.
    fn clone_box(&self, _label: &str) -> Box<dyn Executor> {
        let _ = _label;
        Box::new(FailedExecutor(Error::BackgroundNotSupported))
    }
}

/// A fallback executor that always returns a pre-set error.
///
/// Used as the default `clone_box` implementation for executors that do
/// not support cloning (and therefore cannot be used in parallel mode).
#[derive(Debug)]
pub struct FailedExecutor(pub Error);

impl Executor for FailedExecutor {
    fn execute(&self, _script: &ScriptCode) -> Result<Output, Error> {
        Err(self.0.clone())
    }
}
