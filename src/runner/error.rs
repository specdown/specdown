#[derive(Debug, PartialEq)]
pub enum Error {
    CommandFailed { command: String, message: String },
    ScriptOutputMissing { missing_script_name: String },
    BadShellCommand { command: String, message: String },
}
