use std::collections::HashMap;

use crate::results::{ActionResult, ScriptResult};

pub struct State {
    last_script_result: Option<ScriptResult>,
    script_results: HashMap<String, ScriptResult>,
    is_success: bool,
}

pub trait ScriptOutput {
    fn get_result(&self, name: &str) -> Option<&ScriptResult>;
    fn get_last_result(&self) -> Option<&ScriptResult>;
}

impl State {
    pub fn new() -> Self {
        Self {
            last_script_result: None,
            script_results: HashMap::new(),
            is_success: true,
        }
    }

    pub fn add_result(&mut self, action_result: &ActionResult) {
        if !(action_result.success()) {
            self.is_success = false;
        }

        if let ActionResult::Script(script_result) = action_result {
            let script_name = script_result
                .action
                .script_name
                .clone()
                .map_or("<unknown-script-value".to_string(), Into::into);
            self.script_results
                .insert(script_name, script_result.clone());
            self.last_script_result = Some(script_result.clone());
        }
    }

    pub const fn is_success(&self) -> bool {
        self.is_success
    }
}

impl ScriptOutput for State {
    fn get_result(&self, name: &str) -> Option<&ScriptResult> {
        self.script_results.get(name)
    }

    fn get_last_result(&self) -> Option<&ScriptResult> {
        self.last_script_result.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionResult, ScriptOutput, State};
    use crate::results::{CreateFileResult, ScriptResult, VerifyResult};
    use crate::types::{
        CreateFileAction, ExitCode, FileContent, FilePath, OutputExpectation, ScriptAction,
        ScriptCode, ScriptName, Source, Stream, VerifyAction, VerifyValue,
    };

    #[test]
    fn sets_success_when_initialized() {
        let state = State::new();
        assert!(state.is_success());
    }

    #[test]
    fn does_not_update_success_when_successful_script_result_is_added() {
        let action = ScriptAction {
            script_name: Some(ScriptName("script1".to_string())),
            script_code: ScriptCode("script1".to_string()),
            expected_exit_code: None,
            expected_output: OutputExpectation::Any,
        };
        let script_result1 = ActionResult::Script(ScriptResult {
            action,
            exit_code: Some(ExitCode(0)),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
        });
        let mut state = State::new();
        state.add_result(&script_result1);
        assert!(state.is_success());
    }

    #[test]
    fn does_not_succeed_when_script_failed() {
        let action = ScriptAction {
            script_name: Some(ScriptName("script1".to_string())),
            script_code: ScriptCode("script1".to_string()),
            expected_exit_code: Some(ExitCode(1)),
            expected_output: OutputExpectation::Any,
        };
        let script_result1 = ActionResult::Script(ScriptResult {
            action,
            exit_code: Some(ExitCode(2)),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
        });
        let mut state = State::new();
        state.add_result(&script_result1);
        assert!(!state.is_success());
    }

    #[test]
    fn does_not_update_success_when_file_result_is_added() {
        let action = CreateFileAction {
            file_path: FilePath("example.txt".to_string()),
            file_content: FileContent("".to_string()),
        };
        let file_result = ActionResult::CreateFile(CreateFileResult { action });
        let mut state = State::new();
        state.add_result(&file_result);
        assert!(state.is_success());
    }

    #[test]
    fn get_result_returns_the_result_when_script_result_exists() {
        let script_result1 = ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("script1".to_string())),
                script_code: ScriptCode("script1".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(0)),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
        };
        let script_result2 = ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("script2".to_string())),
                script_code: ScriptCode("script1".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(0)),
            stdout: "stdout2".to_string(),
            stderr: "stderr2".to_string(),
        };
        let mut state = State::new();
        state.add_result(&ActionResult::Script(script_result1.clone()));
        state.add_result(&ActionResult::Script(script_result2.clone()));
        assert_eq!(state.get_result("script1"), Some(&script_result1));
        assert_eq!(state.get_result("script2"), Some(&script_result2));
    }

    #[test]
    fn get_result_returns_none_when_script_result_does_not_exists() {
        let state = State::new();
        assert_eq!(state.get_result("does-not-exist"), None);
    }

    #[test]
    fn does_not_fail_when_verify_was_successful() {
        let verify_result = ActionResult::Verify(VerifyResult {
            action: VerifyAction {
                source: Source {
                    name: Some(ScriptName("script2".to_string())),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected".to_string()),
            },
            got: "expected".to_string(),
        });
        let mut state = State::new();
        state.add_result(&verify_result);
        assert!(state.is_success());
    }

    #[test]
    fn does_not_succeed_when_verify_was_successful_after_failure() {
        let verify_result_failure = ActionResult::Verify(VerifyResult {
            action: VerifyAction {
                source: Source {
                    name: Some(ScriptName("script2".to_string())),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected".to_string()),
            },
            got: "different".to_string(),
        });
        let verify_result_success = ActionResult::Verify(VerifyResult {
            action: VerifyAction {
                source: Source {
                    name: Some(ScriptName("script2".to_string())),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected".to_string()),
            },
            got: "expected".to_string(),
        });
        let mut state = State::new();
        state.add_result(&verify_result_failure);
        state.add_result(&verify_result_success);
        assert!(!state.is_success());
    }

    #[test]
    fn it_fails_when_verify_was_not_successful() {
        let failed_verify_result = ActionResult::Verify(VerifyResult {
            action: VerifyAction {
                source: Source {
                    name: Some(ScriptName("script2".to_string())),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected".to_string()),
            },
            got: "not expected".to_string(),
        });
        let mut state = State::new();
        state.add_result(&failed_verify_result);
        assert!(!state.is_success());
    }

    #[test]
    fn get_last_result_returns_none_when_no_scripts_have_been_run() {
        assert_eq!(None, State::new().get_last_result());
    }

    #[test]
    fn get_last_result_returns_result_from_last_script() {
        let action = ScriptAction {
            script_name: None,
            script_code: ScriptCode("script1".to_string()),
            expected_exit_code: None,
            expected_output: OutputExpectation::Any,
        };
        let script_result = ScriptResult {
            action,
            exit_code: Some(ExitCode(0)),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
        };
        let mut state = State::new();
        state.add_result(&ActionResult::Script(script_result.clone()));
        assert_eq!(Some(&script_result), state.get_last_result());
    }
}
