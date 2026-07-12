use std::path::PathBuf;

use crate::parsers;

#[derive(Debug, Eq, thiserror::Error, PartialEq, Clone)]
pub enum Error {
    #[error("{0}")]
    RunFailed(#[from] parsers::Error),
    #[error("Failed to load config file '{}': {message}", path.display())]
    ConfigFileLoadFailed { path: PathBuf, message: String },
    #[error("Failed to run command: {command} (Error: {message})")]
    CommandFailed { command: String, message: String },
    #[error("Failed to verify the output of '{missing_script_name}': No script with that name has been executed yet.")]
    ScriptOutputMissing { missing_script_name: String },
    #[error("Invalid shell command provided: {command} (Error: {message})")]
    BadShellCommand { command: String, message: String },
    #[error("Background scripts are not supported with this executor")]
    BackgroundNotSupported,
    #[error("The mock server has not been started")]
    MockServerNotStarted,
    #[error("Failed to spawn background process: {message}")]
    SpawnFailed { message: String },
    #[cfg(feature = "container")]
    #[error("The container executor requires Docker, but it is not available: {message}")]
    DockerNotAvailable { message: String },
    #[cfg(feature = "container")]
    #[error("Failed to pull Docker image '{image}': {message}")]
    ImagePullFailed { image: String, message: String },
    #[cfg(not(feature = "container"))]
    #[error("The container executor feature is not enabled. Rebuild specdown with `--features container`")]
    ContainerFeatureNotEnabled,
    /// A `background` block with a `ready_when` condition exited before the
    /// condition became true.
    #[error("Background script '{script_name}' exited with code {exit_code} before the ready_when condition was met")]
    BackgroundExitedBeforeReady { script_name: String, exit_code: i32 },
    /// A `ready_when` condition was not satisfied within the timeout.
    #[error("Background script '{script_name}' did not become ready within {timeout_secs} seconds (ready_when: {condition})")]
    ReadyWhenTimeout {
        script_name: String,
        timeout_secs: u32,
        condition: String,
    },
}
