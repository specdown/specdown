use crate::types::{
    BackgroundAction, CreateFileAction, ExitCode, OutputExpectation, ScriptAction, ScriptName,
    VerifyAction,
};

#[derive(Debug, Eq, PartialEq)]
pub enum ActionError {
    ExitCodeIsIncorrect(ScriptResult),
    UnexpectedOutputIsPresent(ScriptResult),
    OutputDoesNotMatch(VerifyResult),
    BackgroundExitedWithError(BackgroundStopResult),
}

trait ActionErrorProvider {
    fn error(&self) -> Option<ActionError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptResult {
    pub action: ScriptAction,
    pub exit_code: Option<ExitCode>,
    pub stdout: String,
    pub stderr: String,
}

impl ActionErrorProvider for ScriptResult {
    fn error(&self) -> Option<ActionError> {
        if self.action.expected_exit_code.is_some()
            && self.action.expected_exit_code != self.exit_code
        {
            return Some(ActionError::ExitCodeIsIncorrect(self.clone()));
        }

        if self.action.expected_output == OutputExpectation::StdOut && !self.stderr.is_empty() {
            return Some(ActionError::UnexpectedOutputIsPresent(self.clone()));
        }

        if self.action.expected_output == OutputExpectation::StdErr && !self.stdout.is_empty() {
            return Some(ActionError::UnexpectedOutputIsPresent(self.clone()));
        }

        if self.action.expected_output == OutputExpectation::None && !self.stdout.is_empty() {
            return Some(ActionError::UnexpectedOutputIsPresent(self.clone()));
        }

        if self.action.expected_output == OutputExpectation::None && !self.stderr.is_empty() {
            return Some(ActionError::UnexpectedOutputIsPresent(self.clone()));
        }

        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyResult {
    pub action: VerifyAction,
    pub got: String,
}

impl ActionErrorProvider for VerifyResult {
    fn error(&self) -> Option<ActionError> {
        let normalize = |s: &str| s.replace('\r', "");
        let expected = normalize(&String::from(self.action.expected_value.clone()));
        let got = normalize(&self.got);
        if expected == got {
            None
        } else {
            Some(ActionError::OutputDoesNotMatch(self.clone()))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateFileResult {
    pub action: CreateFileAction,
}

impl ActionErrorProvider for CreateFileResult {
    fn error(&self) -> Option<ActionError> {
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackgroundStartResult {
    pub action: BackgroundAction,
}

impl ActionErrorProvider for BackgroundStartResult {
    fn error(&self) -> Option<ActionError> {
        None
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackgroundStopResult {
    pub script_name: Option<ScriptName>,
    pub exit_status: BackgroundExitStatus,
}

impl ActionErrorProvider for BackgroundStopResult {
    fn error(&self) -> Option<ActionError> {
        match self.exit_status {
            BackgroundExitStatus::Exited(code) if i32::from(code) != 0 => {
                Some(ActionError::BackgroundExitedWithError(self.clone()))
            }
            _ => None,
        }
    }
}

/// How a background process ended.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BackgroundExitStatus {
    /// The process was still running and specdown killed it.
    Killed,
    /// The process exited on its own with the given exit code.
    Exited(ExitCode),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionResult {
    Script(ScriptResult),
    Verify(VerifyResult),
    CreateFile(CreateFileResult),
    BackgroundStart(BackgroundStartResult),
    BackgroundStop(BackgroundStopResult),
}

impl ActionResult {
    pub fn success(&self) -> bool {
        self.error().is_none()
    }

    pub fn error(&self) -> Option<ActionError> {
        self.as_error_provider().error()
    }

    fn as_error_provider(&self) -> &dyn ActionErrorProvider {
        match self {
            Self::Script(result) => result,
            Self::Verify(result) => result,
            Self::CreateFile(result) => result,
            Self::BackgroundStart(result) => result,
            Self::BackgroundStop(result) => result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ActionError, ActionResult, BackgroundExitStatus, BackgroundStopResult, CreateFileResult,
        ScriptResult, VerifyResult,
    };

    mod success {
        use super::{
            ActionError, ActionResult, BackgroundExitStatus, BackgroundStopResult,
            CreateFileResult, ScriptResult, VerifyResult,
        };

        mod error {
            use super::{ActionError, ActionResult, ScriptResult};
            use crate::types::{ExitCode, OutputExpectation, ScriptAction, ScriptCode, ScriptName};

            #[test]
            fn returns_none_when_successful_script() {
                let result = ActionResult::Script(ScriptResult {
                    action: ScriptAction {
                        script_name: Some(ScriptName("example_script".to_string())),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: None,
                        expected_output: OutputExpectation::Any,
                    },
                    exit_code: None,
                    stdout: String::new(),
                    stderr: String::new(),
                });
                assert_eq!(result.error(), None);
                assert!(result.success());
            }

            #[test]
            fn returns_none_when_exit_code_is_expected() {
                let result = ActionResult::Script(ScriptResult {
                    action: ScriptAction {
                        script_name: Some(ScriptName("example_script".to_string())),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: Some(ExitCode(1)),
                        expected_output: OutputExpectation::Any,
                    },
                    exit_code: Some(ExitCode(1)),
                    stdout: String::new(),
                    stderr: String::new(),
                });
                assert_eq!(result.error(), None);
                assert!(result.success());
            }

            #[test]
            fn returns_exit_code_is_incorrect_when_exit_code_is_incorrect() {
                let script_result = ScriptResult {
                    action: ScriptAction {
                        script_name: Some(ScriptName("example_script".to_string())),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: Some(ExitCode(1)),
                        expected_output: OutputExpectation::Any,
                    },
                    exit_code: Some(ExitCode(2)),
                    stdout: String::new(),
                    stderr: String::new(),
                };
                let result = ActionResult::Script(script_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::ExitCodeIsIncorrect(script_result))
                );
                assert!(!result.success());
            }

            #[test]
            fn returns_unexpected_output_is_present_when_stderr_is_present_but_only_stdout_is_expected(
            ) {
                let script_result = ScriptResult {
                    action: ScriptAction {
                        script_name: Some(ScriptName("example_script".to_string())),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: None,
                        expected_output: OutputExpectation::StdOut,
                    },
                    exit_code: None,
                    stdout: String::new(),
                    stderr: "unexpected output".to_string(),
                };
                let result = ActionResult::Script(script_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::UnexpectedOutputIsPresent(script_result))
                );
                assert!(!result.success());
            }

            #[test]
            fn returns_unexpected_output_is_present_when_stdout_is_present_but_only_stderr_is_expected(
            ) {
                let script_result = ScriptResult {
                    action: ScriptAction {
                        script_name: Some(ScriptName("example_script".to_string())),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: None,
                        expected_output: OutputExpectation::StdErr,
                    },
                    exit_code: None,
                    stdout: "unexpected output".to_string(),
                    stderr: String::new(),
                };
                let result = ActionResult::Script(script_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::UnexpectedOutputIsPresent(script_result))
                );
                assert!(!result.success());
            }

            #[test]
            fn returns_unexpected_output_is_present_when_stdout_is_present_but_no_output_is_expected(
            ) {
                let script_result = ScriptResult {
                    action: ScriptAction {
                        script_name: Some(ScriptName("example_script".to_string())),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: None,
                        expected_output: OutputExpectation::None,
                    },
                    exit_code: None,
                    stdout: "unexpected output".to_string(),
                    stderr: String::new(),
                };
                let result = ActionResult::Script(script_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::UnexpectedOutputIsPresent(script_result))
                );
                assert!(!result.success());
            }

            #[test]
            fn returns_unexpected_output_is_present_when_stderr_is_present_but_no_output_is_expected(
            ) {
                let script_result = ScriptResult {
                    action: ScriptAction {
                        script_name: Some(ScriptName("example_script".to_string())),
                        script_code: ScriptCode("example code".to_string()),
                        expected_exit_code: None,
                        expected_output: OutputExpectation::None,
                    },
                    exit_code: None,
                    stdout: String::new(),
                    stderr: "unexpected output".to_string(),
                };
                let result = ActionResult::Script(script_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::UnexpectedOutputIsPresent(script_result))
                );
                assert!(!result.success());
            }
        }

        mod verify {
            use super::{ActionError, ActionResult, VerifyResult};
            use crate::types::{ScriptName, Source, Stream, VerifyAction, VerifyValue};

            #[test]
            fn returns_true_when_expected_output_is_the_same_as_got_output() {
                let result = ActionResult::Verify(VerifyResult {
                    action: VerifyAction {
                        source: Source {
                            name: Some(ScriptName("example_script".to_string())),
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
                            name: Some(ScriptName("example_script".to_string())),
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
            use super::{ActionResult, CreateFileResult};
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

        mod background_stop {
            use super::{ActionError, ActionResult, BackgroundExitStatus, BackgroundStopResult};
            use crate::types::{ExitCode, ScriptName};

            #[test]
            fn returns_none_when_process_was_killed() {
                let result = ActionResult::BackgroundStop(BackgroundStopResult {
                    script_name: Some(ScriptName("server".to_string())),
                    exit_status: BackgroundExitStatus::Killed,
                });
                assert_eq!(result.error(), None);
                assert!(result.success());
            }

            #[test]
            fn returns_none_when_process_exited_with_zero() {
                let result = ActionResult::BackgroundStop(BackgroundStopResult {
                    script_name: Some(ScriptName("server".to_string())),
                    exit_status: BackgroundExitStatus::Exited(ExitCode(0)),
                });
                assert_eq!(result.error(), None);
                assert!(result.success());
            }

            #[test]
            fn returns_error_when_process_exited_with_non_zero() {
                let stop_result = BackgroundStopResult {
                    script_name: Some(ScriptName("server".to_string())),
                    exit_status: BackgroundExitStatus::Exited(ExitCode(1)),
                };
                let result = ActionResult::BackgroundStop(stop_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::BackgroundExitedWithError(stop_result))
                );
                assert!(!result.success());
            }

            #[test]
            fn returns_error_when_process_exited_with_signal_code() {
                let stop_result = BackgroundStopResult {
                    script_name: None,
                    exit_status: BackgroundExitStatus::Exited(ExitCode(134)),
                };
                let result = ActionResult::BackgroundStop(stop_result.clone());
                assert_eq!(
                    result.error(),
                    Some(ActionError::BackgroundExitedWithError(stop_result))
                );
                assert!(!result.success());
            }
        }
    }
}
