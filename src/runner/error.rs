#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    RunFailed { message: String },
    CommandFailed { command: String, message: String },
    ScriptOutputMissing { missing_script_name: String },
    BadShellCommand { command: String, message: String },
}
