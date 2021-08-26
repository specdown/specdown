use crate::types::{CreateFileAction, ScriptAction, VerifyAction, VerifyValue};

#[derive(Clone, Debug, PartialEq)]
pub enum ActionResult {
    Script {
        action: ScriptAction,
        exit_code: Option<i32>,
        stdout: String,
        stderr: String,
    },
    Verify {
        action: VerifyAction,
        got: String,
    },
    CreateFile {
        action: CreateFileAction,
    },
}

impl ActionResult {
    pub fn success(&self) -> bool {
        match self {
            ActionResult::Script {
                action: ScriptAction {
                    expected_exit_code, ..
                },
                exit_code,
                ..
            } => {
                let i32_exit_code = expected_exit_code.map(i32::from);
                i32_exit_code == None || i32_exit_code == *exit_code
            }
            ActionResult::Verify {
                action: VerifyAction { expected_value, .. },
                got,
                ..
            } => *expected_value == VerifyValue(got.clone()),
            ActionResult::CreateFile { .. } => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ActionResult;

    mod success {
        use super::ActionResult;

        mod script {
            use super::ActionResult;
            use crate::types::{ExitCode, ScriptAction, ScriptCode, ScriptName};

            #[test]
            fn returns_true_when_successful_script() {
                let result = ActionResult::Script {
                    action: ScriptAction {
                        script_name: ScriptName("example_script".to_string()),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: None,
                    },
                    exit_code: None,
                    stdout: "".to_string(),
                    stderr: "".to_string(),
                };
                assert!(result.success());
            }

            #[test]
            fn returns_true_when_exit_code_is_expected() {
                let result = ActionResult::Script {
                    action: ScriptAction {
                        script_name: ScriptName("example_script".to_string()),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: Some(ExitCode(1)),
                    },
                    exit_code: Some(1),
                    stdout: "".to_string(),
                    stderr: "".to_string(),
                };
                assert!(result.success());
            }

            #[test]
            fn returns_false_when_exit_code_is_incorrect() {
                let result = ActionResult::Script {
                    action: ScriptAction {
                        script_name: ScriptName("example_script".to_string()),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: Some(ExitCode(1)),
                    },
                    exit_code: Some(2),
                    stdout: "".to_string(),
                    stderr: "".to_string(),
                };
                assert!(!result.success());
            }
        }

        mod verify {
            use super::ActionResult;
            use crate::types::{ScriptName, Source, Stream, VerifyAction, VerifyValue};

            #[test]
            fn returns_true_when_expected_output_is_the_same_as_got_output() {
                let result = ActionResult::Verify {
                    action: VerifyAction {
                        source: Source {
                            name: ScriptName("example_script".to_string()),
                            stream: Stream::StdOut,
                        },
                        expected_value: VerifyValue("the output".to_string()),
                    },
                    got: "the output".to_string(),
                };
                assert!(result.success());
            }

            #[test]
            fn returns_false_when_expected_output_is_not_the_same_as_got_output() {
                let result = ActionResult::Verify {
                    action: VerifyAction {
                        source: Source {
                            name: ScriptName("example_script".to_string()),
                            stream: Stream::StdOut,
                        },
                        expected_value: VerifyValue("expected output".to_string()),
                    },
                    got: "different output".to_string(),
                };
                assert!(!result.success());
            }
        }

        mod create_file {
            use super::ActionResult;
            use crate::types::{CreateFileAction, FileContent, FilePath};

            #[test]
            fn returns_true() {
                let result = ActionResult::CreateFile {
                    action: CreateFileAction {
                        file_path: FilePath("path".to_string()),
                        file_content: FileContent("content".to_string()),
                    },
                };
                assert!(result.success());
            }
        }
    }
}
