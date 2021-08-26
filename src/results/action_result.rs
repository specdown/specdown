use crate::types::{ScriptAction, VerifyAction};

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
        action: VerifyAction,
        got: String,
        success: bool,
    },
    File {
        path: String,
    },
}
