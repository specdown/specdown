use std::collections::HashMap;

use crate::results::{ActionResult, ScriptResult};

pub struct State {
    last_script_result: Option<ScriptResult>,
    script_results: HashMap<String, ScriptResult>,
    is_success: bool,
}

pub trait ScriptOutput {
    fn get_stdout(&self, name: &str) -> Option<String>;
    fn get_stderr(&self, name: &str) -> Option<String>;
    fn get_last_stdout(&self) -> Option<String>;
    fn get_last_stderr(&self) -> Option<String>;
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
                .map_or("<unknown-script-value".to_string(), |value| value.into());
            self.script_results
                .insert(script_name, script_result.clone());
            self.last_script_result = Some(script_result.clone());
        }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}

impl ScriptOutput for State {
    fn get_stdout(&self, name: &str) -> Option<String> {
        self.script_results
            .get(name)
            .map(|result| result.stdout.to_string())
    }

    fn get_stderr(&self, name: &str) -> Option<String> {
        self.script_results
            .get(name)
            .map(|result| result.stderr.to_string())
    }

    fn get_last_stdout(&self) -> Option<String> {
        self.last_script_result.clone().map(|result| result.stdout)
    }

    fn get_last_stderr(&self) -> Option<String> {
        self.last_script_result.clone().map(|result| result.stderr)
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
    fn get_script_stdout_returns_the_output_when_script_output_exists() {
        let script_result1 = ActionResult::Script(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("script1".to_string())),
                script_code: ScriptCode("script1".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(0)),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
        });
        let script_result2 = ActionResult::Script(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("script2".to_string())),
                script_code: ScriptCode("script2".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(0)),
            stdout: "stdout2".to_string(),
            stderr: "stderr2".to_string(),
        });
        let mut state = State::new();
        state.add_result(&script_result1);
        state.add_result(&script_result2);
        assert_eq!(state.get_stdout("script1"), Some("stdout1".to_string()));
        assert_eq!(state.get_stdout("script2"), Some("stdout2".to_string()));
    }

    #[test]
    fn get_script_stdout_returns_none_when_script_output_does_not_exists() {
        let state = State::new();
        assert_eq!(state.get_stdout("does-not-exist"), None);
    }

    #[test]
    fn get_script_stderr_returns_the_output_when_script_output_exists() {
        let script_result1 = ActionResult::Script(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("script1".to_string())),
                script_code: ScriptCode("script1".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(0)),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
        });
        let script_result2 = ActionResult::Script(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("script2".to_string())),
                script_code: ScriptCode("script1".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(0)),
            stdout: "stdout2".to_string(),
            stderr: "stderr2".to_string(),
        });
        let mut state = State::new();
        state.add_result(&script_result1);
        state.add_result(&script_result2);
        assert_eq!(state.get_stderr("script1"), Some("stderr1".to_string()));
        assert_eq!(state.get_stderr("script2"), Some("stderr2".to_string()));
    }

    #[test]
    fn get_script_stderr_returns_none_when_script_output_does_not_exists() {
        let state = State::new();
        assert_eq!(state.get_stderr("does-not-exist"), None);
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
    fn get_last_stdout_returns_none_when_no_scripts_have_been_run() {
        assert_eq!(None, State::new().get_last_stdout());
    }

    #[test]
    fn get_last_stdout_returns_none_stdout_from_last_script() {
        let action = ScriptAction {
            script_name: None,
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
        assert_eq!(Some("stdout1".to_string()), state.get_last_stdout());
    }

    #[test]
    fn get_last_stderr_returns_none_when_no_scripts_have_been_run() {
        assert_eq!(None, State::new().get_last_stderr());
    }

    #[test]
    fn get_last_stderr_returns_none_stdout_from_last_script() {
        let action = ScriptAction {
            script_name: None,
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
        assert_eq!(Some("stderr1".to_string()), state.get_last_stderr());
    }
}
