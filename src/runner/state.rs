use std::collections::HashMap;

use crate::results::action_result::ActionResult;
use crate::types::{ScriptAction, ScriptName};

pub struct State {
    script_results: HashMap<String, ActionResult>,
    is_success: bool,
}

pub trait ScriptOutput {
    fn get_stdout(&self, name: &str) -> Option<&str>;
    fn get_stderr(&self, name: &str) -> Option<&str>;
}

impl State {
    pub fn new() -> Self {
        Self {
            script_results: HashMap::new(),
            is_success: true,
        }
    }

    pub fn add_result(&mut self, action_result: &ActionResult) {
        if !(action_result.success()) {
            self.is_success = false;
        }

        if let ActionResult::Script {
            action:
                ScriptAction {
                    script_name: ScriptName(name),
                    ..
                },
            ..
        } = action_result
        {
            self.script_results
                .insert(name.to_string(), (*action_result).clone());
        }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}

impl ScriptOutput for State {
    fn get_stdout(&self, name: &str) -> Option<&str> {
        self.script_results
            .get(name)
            .and_then(|result| match result {
                ActionResult::Script { stdout, .. } => Some(&stdout[..]),
                _ => panic!("Only TestResult::Script results should be stored in the state"),
            })
    }

    fn get_stderr(&self, name: &str) -> Option<&str> {
        self.script_results
            .get(name)
            .and_then(|result| match result {
                ActionResult::Script { stderr, .. } => Some(&stderr[..]),
                _ => panic!("Only TestResult::Script results should be stored in the state"),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionResult, ScriptOutput, State};
    use crate::types::{
        CreateFileAction, ExitCode, FileContent, FilePath, ScriptAction, ScriptCode, ScriptName,
        Source, Stream, VerifyAction, VerifyValue,
    };

    #[test]
    fn sets_success_when_initialized() {
        let state = State::new();
        assert!(state.is_success());
    }

    #[test]
    fn does_not_update_success_when_successful_script_result_is_added() {
        let action = ScriptAction {
            script_name: ScriptName("script1".to_string()),
            script_code: ScriptCode("script1".to_string()),
            expected_exit_code: None,
        };
        let script_result1 = ActionResult::Script {
            action,
            exit_code: Some(0),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        assert!(state.is_success());
    }

    #[test]
    fn does_not_succeed_when_script_failed() {
        let action = ScriptAction {
            script_name: ScriptName("script1".to_string()),
            script_code: ScriptCode("script1".to_string()),
            expected_exit_code: Some(ExitCode(1)),
        };
        let script_result1 = ActionResult::Script {
            action,
            exit_code: Some(2),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
        };
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
        let file_result = ActionResult::CreateFile { action };
        let mut state = State::new();
        state.add_result(&file_result);
        assert!(state.is_success());
    }

    #[test]
    fn get_script_stdout_returns_the_output_when_script_output_exists() {
        let script_result1 = ActionResult::Script {
            action: ScriptAction {
                script_name: ScriptName("script1".to_string()),
                script_code: ScriptCode("script1".to_string()),
                expected_exit_code: None,
            },
            exit_code: Some(0),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
        };
        let script_result2 = ActionResult::Script {
            action: ScriptAction {
                script_name: ScriptName("script2".to_string()),
                script_code: ScriptCode("script2".to_string()),
                expected_exit_code: None,
            },
            exit_code: Some(0),
            stdout: "stdout2".to_string(),
            stderr: "stderr2".to_string(),
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        state.add_result(&script_result2);
        assert_eq!(state.get_stdout("script1"), Some("stdout1"));
        assert_eq!(state.get_stdout("script2"), Some("stdout2"));
    }

    #[test]
    fn get_script_stdout_returns_none_when_script_output_does_not_exists() {
        let state = State::new();
        assert_eq!(state.get_stdout("does-not-exist"), None);
    }

    #[test]
    fn get_script_stderr_returns_the_output_when_script_output_exists() {
        let script_result1 = ActionResult::Script {
            action: ScriptAction {
                script_name: ScriptName("script1".to_string()),
                script_code: ScriptCode("script1".to_string()),
                expected_exit_code: None,
            },
            exit_code: Some(0),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
        };
        let script_result2 = ActionResult::Script {
            action: ScriptAction {
                script_name: ScriptName("script2".to_string()),
                script_code: ScriptCode("script1".to_string()),
                expected_exit_code: None,
            },
            exit_code: Some(0),
            stdout: "stdout2".to_string(),
            stderr: "stderr2".to_string(),
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        state.add_result(&script_result2);
        assert_eq!(state.get_stderr("script1"), Some("stderr1"));
        assert_eq!(state.get_stderr("script2"), Some("stderr2"));
    }

    #[test]
    fn get_script_stderr_returns_none_when_script_output_does_not_exists() {
        let state = State::new();
        assert_eq!(state.get_stderr("does-not-exist"), None);
    }

    #[test]
    fn does_not_fail_when_verify_was_successful() {
        let verify_result = ActionResult::Verify {
            action: VerifyAction {
                source: Source {
                    name: ScriptName("script2".to_string()),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected".to_string()),
            },
            got: "expected".to_string(),
        };
        let mut state = State::new();
        state.add_result(&verify_result);
        assert!(state.is_success());
    }

    #[test]
    fn does_not_succeed_when_verify_was_successful_after_failure() {
        let verify_result_failure = ActionResult::Verify {
            action: VerifyAction {
                source: Source {
                    name: ScriptName("script2".to_string()),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected".to_string()),
            },
            got: "different".to_string(),
        };
        let verify_result_success = ActionResult::Verify {
            action: VerifyAction {
                source: Source {
                    name: ScriptName("script2".to_string()),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected".to_string()),
            },
            got: "expected".to_string(),
        };
        let mut state = State::new();
        state.add_result(&verify_result_failure);
        state.add_result(&verify_result_success);
        assert!(!state.is_success());
    }

    #[test]
    fn it_fails_when_verify_was_not_successful() {
        let failed_verify_result = ActionResult::Verify {
            action: VerifyAction {
                source: Source {
                    name: ScriptName("script2".to_string()),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected".to_string()),
            },
            got: "not expected".to_string(),
        };
        let mut state = State::new();
        state.add_result(&failed_verify_result);
        assert!(!state.is_success());
    }
}
