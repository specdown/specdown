use std::collections::HashMap;

use crate::results::test_result::TestResult;

pub struct State {
    script_results: HashMap<String, TestResult>,
    is_success: bool,
    number_succeed: u32,
    number_failed: u32,
}

#[derive(Debug, PartialEq)]
pub struct Summary {
    pub number_succeeded: u32,
    pub number_failed: u32,
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
            number_succeed: 0,
            number_failed: 0,
        }
    }

    pub fn add_result(&mut self, test_result: &TestResult) {
        match test_result {
            TestResult::Script { name, success, .. } => {
                self.script_results
                    .insert(name.to_string(), (*test_result).clone());
                if *success {
                    self.number_succeed += 1
                } else {
                    self.is_success = *success;
                    self.number_failed += 1;
                }
            }
            TestResult::Verify { success, .. } => {
                if *success {
                    self.number_succeed += 1
                } else {
                    self.is_success = *success;
                    self.number_failed += 1
                }
            }
            TestResult::File { .. } => self.number_succeed += 1,
        }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }

    pub fn summary(&self) -> Summary {
        Summary {
            number_succeeded: self.number_succeed,
            number_failed: self.number_failed,
        }
    }
}

impl ScriptOutput for State {
    fn get_stdout(&self, name: &str) -> Option<&str> {
        self.script_results
            .get(name)
            .and_then(|result| match result {
                TestResult::Script { stdout, .. } => Some(&stdout[..]),
                _ => panic!("Only TestResult::Script results should be stored in the state"),
            })
    }

    fn get_stderr(&self, name: &str) -> Option<&str> {
        self.script_results
            .get(name)
            .and_then(|result| match result {
                TestResult::Script { stderr, .. } => Some(&stderr[..]),
                _ => panic!("Only TestResult::Script results should be stored in the state"),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{ScriptOutput, State, TestResult};
    use crate::runner::state::Summary;

    #[test]
    fn sets_success_when_initialized() {
        let state = State::new();
        assert_eq!(state.is_success(), true);
    }

    #[test]
    fn sets_summary_to_zero_when_initialized() {
        let state = State::new();
        assert_eq!(
            state.summary(),
            Summary {
                number_succeeded: 0,
                number_failed: 0
            }
        );
    }

    #[test]
    fn does_not_update_success_when_successful_script_result_is_added() {
        let script_result1 = TestResult::Script {
            name: "script1".to_string(),
            exit_code: Some(0),
            expected_exit_code: None,
            script: "script1".to_string(),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        assert_eq!(state.is_success(), true);
    }

    #[test]
    fn updates_success_count_when_successful_script_result_is_added() {
        let script_result1 = TestResult::Script {
            name: "script1".to_string(),
            exit_code: Some(0),
            expected_exit_code: None,
            script: "script1".to_string(),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        assert_eq!(
            state.summary(),
            Summary {
                number_succeeded: 1,
                number_failed: 0
            }
        );
    }

    #[test]
    fn does_not_succeed_when_script_failed() {
        let script_result1 = TestResult::Script {
            name: "script1".to_string(),
            exit_code: Some(0),
            expected_exit_code: Some(1),
            script: "script1".to_string(),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
            success: false,
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        assert_eq!(state.is_success(), false);
    }

    #[test]
    fn updates_failed_count_when_successful_script_result_is_added() {
        let script_result1 = TestResult::Script {
            name: "script1".to_string(),
            exit_code: Some(0),
            expected_exit_code: None,
            script: "script1".to_string(),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
            success: false,
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        assert_eq!(
            state.summary(),
            Summary {
                number_succeeded: 0,
                number_failed: 1
            }
        );
    }

    #[test]
    fn does_not_update_success_when_file_result_is_added() {
        let file_result = TestResult::File {
            path: "example.txt".to_string(),
        };
        let mut state = State::new();
        state.add_result(&file_result);
        assert_eq!(state.is_success(), true);
    }

    #[test]
    fn updates_success_count_when_file_result_is_added() {
        let file_result = TestResult::File {
            path: "example.txt".to_string(),
        };
        let mut state = State::new();
        state.add_result(&file_result);
        assert_eq!(
            state.summary(),
            Summary {
                number_succeeded: 1,
                number_failed: 0
            }
        );
    }

    #[test]
    fn get_script_stdout_returns_the_output_when_script_output_exists() {
        let script_result1 = TestResult::Script {
            name: "script1".to_string(),
            exit_code: Some(0),
            expected_exit_code: None,
            script: "script1".to_string(),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
            success: true,
        };
        let script_result2 = TestResult::Script {
            name: "script2".to_string(),
            exit_code: Some(0),
            expected_exit_code: None,
            script: "script1".to_string(),
            stdout: "stdout2".to_string(),
            stderr: "stderr2".to_string(),
            success: true,
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
        let script_result1 = TestResult::Script {
            name: "script1".to_string(),
            exit_code: Some(0),
            expected_exit_code: None,
            script: "script1".to_string(),
            stdout: "stdout1".to_string(),
            stderr: "stderr1".to_string(),
            success: true,
        };
        let script_result2 = TestResult::Script {
            name: "script2".to_string(),
            exit_code: Some(0),
            expected_exit_code: None,
            script: "script1".to_string(),
            stdout: "stdout2".to_string(),
            stderr: "stderr2".to_string(),
            success: true,
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
        let verify_result = TestResult::Verify {
            script_name: "script2".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&verify_result);
        assert_eq!(state.is_success(), true);
    }

    #[test]
    fn updates_success_count_when_verify_was_successful() {
        let verify_result = TestResult::Verify {
            script_name: "script2".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&verify_result);
        assert_eq!(
            state.summary(),
            Summary {
                number_succeeded: 1,
                number_failed: 0
            }
        );
    }

    #[test]
    fn does_not_succeed_when_verify_was_successful_after_failure() {
        let verify_result_failure = TestResult::Verify {
            script_name: "script1".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: false,
        };
        let verify_result_success = TestResult::Verify {
            script_name: "script2".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&verify_result_failure);
        state.add_result(&verify_result_success);
        assert_eq!(state.is_success(), false);
    }

    #[test]
    fn it_fails_when_verify_was_not_successful() {
        let verify_result = TestResult::Verify {
            script_name: "script2".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: false,
        };
        let mut state = State::new();
        state.add_result(&verify_result);
        assert_eq!(state.is_success(), false);
    }

    #[test]
    fn updates_fail_count_when_verify_was_not_successful() {
        let verify_result = TestResult::Verify {
            script_name: "script2".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: false,
        };
        let mut state = State::new();
        state.add_result(&verify_result);
        assert_eq!(
            state.summary(),
            Summary {
                number_succeeded: 0,
                number_failed: 1
            }
        );
    }
}
