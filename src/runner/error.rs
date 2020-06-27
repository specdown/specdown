#[derive(Debug, PartialEq)]
pub enum Error {
    CommandFailed,
    ScriptOutputMissing { missing_script_name: String },
}
