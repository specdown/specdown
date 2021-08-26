use crate::types::{CreateFileAction, ScriptAction, VerifyAction, VerifyValue};

#[derive(Debug, PartialEq)]
pub enum ActionError {
    ExitCodeIsIncorrect(ScriptResult),
    OutputDoesNotMatch(VerifyResult),
}

trait ActionErrorProvider {
    fn error(&self) -> Option<ActionError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptResult {
    pub action: ScriptAction,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl ActionErrorProvider for ScriptResult {
    fn error(&self) -> Option<ActionError> {
        let i32_exit_code = self.action.expected_exit_code.map(i32::from);
        if i32_exit_code != None && i32_exit_code != self.exit_code {
            Some(ActionError::ExitCodeIsIncorrect(self.clone()))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VerifyResult {
    pub action: VerifyAction,
    pub got: String,
}

impl ActionErrorProvider for VerifyResult {
    fn error(&self) -> Option<ActionError> {
        if self.action.expected_value == VerifyValue(self.got.clone()) {
            None
        } else {
            Some(ActionError::OutputDoesNotMatch(self.clone()))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateFileResult {
    pub action: CreateFileAction,
}

impl ActionErrorProvider for CreateFileResult {
    fn error(&self) -> Option<ActionError> {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ActionResult {
    Script(ScriptResult),
    Verify(VerifyResult),
    CreateFile(CreateFileResult),
}

impl ActionResult {
    pub fn success(&self) -> bool {
        self.error() == None
    }

    pub fn error(&self) -> Option<ActionError> {
        self.as_error_provider().error()
    }

    fn as_error_provider(&self) -> &dyn ActionErrorProvider {
        match self {
            ActionResult::Script(result) => result,
            ActionResult::Verify(result) => result,
            ActionResult::CreateFile(result) => result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionError, ActionResult};

    mod success {
        use super::{ActionError, ActionResult};

        mod error {
            use super::{ActionError, ActionResult};
            use crate::results::action_result::ScriptResult;
            use crate::types::{ExitCode, ScriptAction, ScriptCode, ScriptName};

            #[test]
            fn returns_none_when_successful_script() {
                let result = ActionResult::Script(ScriptResult {
                    action: ScriptAction {
                        script_name: ScriptName("example_script".to_string()),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: None,
                    },
                    exit_code: None,
                    stdout: "".to_string(),
                    stderr: "".to_string(),
                });
                assert_eq!(result.error(), None);
                assert!(result.success());
            }

            #[test]
            fn returns_none_when_exit_code_is_expected() {
                let result = ActionResult::Script(ScriptResult {
                    action: ScriptAction {
                        script_name: ScriptName("example_script".to_string()),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: Some(ExitCode(1)),
                    },
                    exit_code: Some(1),
                    stdout: "".to_string(),
                    stderr: "".to_string(),
                });
                assert_eq!(result.error(), None);
                assert!(result.success());
            }

            #[test]
            fn returns_exit_code_is_incorrect_when_exit_code_is_incorrect() {
                let script_result = ScriptResult {
                    action: ScriptAction {
                        script_name: ScriptName("example_script".to_string()),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: Some(ExitCode(1)),
                    },
                    exit_code: Some(2),
                    stdout: "".to_string(),
                    stderr: "".to_string(),
                };
                let result = ActionResult::Script(script_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::ExitCodeIsIncorrect(script_result))
                );
                assert!(!result.success());
            }
        }

        mod verify {
            use super::{ActionError, ActionResult};
            use crate::results::action_result::VerifyResult;
            use crate::types::{ScriptName, Source, Stream, VerifyAction, VerifyValue};

            #[test]
            fn returns_true_when_expected_output_is_the_same_as_got_output() {
                let result = ActionResult::Verify(VerifyResult {
                    action: VerifyAction {
                        source: Source {
                            name: ScriptName("example_script".to_string()),
                            stream: Stream::StdOut,
                        },
                        expected_value: VerifyValue("the output".to_string()),
                    },
                    got: "the output".to_string(),
                });
                assert_eq!(result.error(), None);
                assert!(result.success());
            }

            #[test]
            fn returns_false_when_expected_output_is_not_the_same_as_got_output() {
                let verify_result = VerifyResult {
                    action: VerifyAction {
                        source: Source {
                            name: ScriptName("example_script".to_string()),
                            stream: Stream::StdOut,
                        },
                        expected_value: VerifyValue("expected output".to_string()),
                    },
                    got: "different output".to_string(),
                };
                let result = ActionResult::Verify(verify_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::OutputDoesNotMatch(verify_result))
                );
                assert!(!result.success());
            }
        }

        mod create_file {
            use super::ActionResult;
            use crate::results::action_result::CreateFileResult;
            use crate::types::{CreateFileAction, FileContent, FilePath};

            #[test]
            fn returns_true() {
                let result = ActionResult::CreateFile(CreateFileResult {
                    action: CreateFileAction {
                        file_path: FilePath("path".to_string()),
                        file_content: FileContent("content".to_string()),
                    },
                });
                assert!(result.success());
            }
        }
    }
}
