use crate::types::{CreateFileAction, ExitCode, ScriptAction, VerifyAction, VerifyValue};

#[derive(Debug, PartialEq)]
pub enum ActionError {
    ExitCodeIsIncorrect {
        expected_exit_code: ExitCode,
        actual_exit_code: ExitCode,
    },
    OutputDoesNotMatch,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptResult {
    pub action: ScriptAction,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VerifyResult {
    pub action: VerifyAction,
    pub got: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateFileResult {
    pub action: CreateFileAction,
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
        match self {
            ActionResult::Script(ScriptResult {
                action: ScriptAction {
                    expected_exit_code, ..
                },
                exit_code,
                ..
            }) => {
                let i32_exit_code = expected_exit_code.map(i32::from);
                if i32_exit_code != None && i32_exit_code != *exit_code {
                    Some(ActionError::ExitCodeIsIncorrect {
                        expected_exit_code: expected_exit_code.unwrap(),
                        actual_exit_code: ExitCode(exit_code.unwrap()),
                    })
                } else {
                    None
                }
            }
            ActionResult::Verify(VerifyResult {
                action: VerifyAction { expected_value, .. },
                got,
                ..
            }) => {
                if *expected_value == VerifyValue(got.clone()) {
                    None
                } else {
                    Some(ActionError::OutputDoesNotMatch)
                }
            }
            ActionResult::CreateFile(_) => None,
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
                let result = ActionResult::Script(ScriptResult {
                    action: ScriptAction {
                        script_name: ScriptName("example_script".to_string()),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: Some(ExitCode(1)),
                    },
                    exit_code: Some(2),
                    stdout: "".to_string(),
                    stderr: "".to_string(),
                });
                assert_eq!(
                    result.error(),
                    Some(ActionError::ExitCodeIsIncorrect {
                        expected_exit_code: ExitCode(1),
                        actual_exit_code: ExitCode(2)
                    })
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
                let result = ActionResult::Verify(VerifyResult {
                    action: VerifyAction {
                        source: Source {
                            name: ScriptName("example_script".to_string()),
                            stream: Stream::StdOut,
                        },
                        expected_value: VerifyValue("expected output".to_string()),
                    },
                    got: "different output".to_string(),
                });
                assert_eq!(result.error(), Some(ActionError::OutputDoesNotMatch));
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
