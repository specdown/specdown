#[derive(Debug, Eq, thiserror::Error, PartialEq, Clone)]
pub enum Error {
    #[error("{message}")]
    RunFailed { message: String },
    #[error("Failed to run command: {command} (Error: {message})")]
    CommandFailed { command: String, message: String },
    #[error("Failed to verify the output of '{missing_script_name}': No script with that name has been executed yet.")]
    ScriptOutputMissing { missing_script_name: String },
    #[error("Invalid shell command provided: {command} (Error: {message})")]
    BadShellCommand { command: String, message: String },
}
