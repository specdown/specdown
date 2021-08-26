use crate::types::ScriptAction;

#[derive(Clone, Debug, PartialEq)]
pub enum ActionResult {
    Script {
        action: ScriptAction,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
        success: bool,
    },
    Verify {
        script_name: String,
        stream: String,
        expected: String,
        got: String,
        success: bool,
    },
    File {
        path: String,
    },
}
