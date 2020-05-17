use std::collections::HashMap;

use super::test_result::TestResult;

pub struct State {
    number_of_scripts: u32,
    number_of_verifies: u32,
    script_results: HashMap<String, String>,
    is_success: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            number_of_scripts: 0,
            number_of_verifies: 0,
            script_results: HashMap::new(),
            is_success: true,
        }
    }

    pub fn number_of_scripts(&self) -> u32 {
        self.number_of_scripts
    }

    pub fn number_of_verifies(&self) -> u32 {
        self.number_of_verifies
    }

    pub fn add_result(&mut self, test_result: &TestResult) {
        self.number_of_scripts += 1;
        self.number_of_verifies += 1;

        match test_result {
            TestResult::ScriptResult { name, output, .. } => {
                self.script_results
                    .insert(name.to_string(), output.to_string());
            }
            TestResult::VerifyResult { success, .. } => {
                if !success {
                    self.is_success = *success;
                }
            }
        }
    }

    pub fn get_script_output(&self, name: &str) -> Option<&str> {
        self.script_results.get(name).map(|s| &s[..])
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sets_counts_to_zero_when_new_state() {
        let state = State::new();
        assert_eq!(state.number_of_scripts(), 0);
        assert_eq!(state.number_of_verifies(), 0);
    }

    #[test]
    fn sets_success_zero_when_new_state() {
        let state = State::new();
        assert_eq!(state.is_success(), true);
    }

    #[test]
    fn increments_number_of_scripts_when_script_result_is_added() {
        let script_result1 = TestResult::ScriptResult {
            name: "script1".to_string(),
            exit_code: 0,
            script: "script1".to_string(),
            output: "output1".to_string(),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
            success: true,
        };
        let script_result2 = TestResult::ScriptResult {
            name: "script2".to_string(),
            exit_code: 0,
            script: "script1".to_string(),
            output: "output2".to_string(),
            stdout: "stderr2".to_string(),
            stderr: "stderr2".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        assert_eq!(state.number_of_scripts(), 1);
        state.add_result(&script_result2);
        assert_eq!(state.number_of_scripts(), 2);
    }

    #[test]
    fn does_not_update_success_when_script_result_is_added() {
        let script_result1 = TestResult::ScriptResult {
            name: "script1".to_string(),
            exit_code: 0,
            script: "script1".to_string(),
            output: "output1".to_string(),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        assert_eq!(state.is_success(), true);
    }

    #[test]
    fn returns_the_output_when_script_output_exists() {
        let script_result1 = TestResult::ScriptResult {
            name: "script1".to_string(),
            exit_code: 0,
            script: "script1".to_string(),
            output: "output1".to_string(),
            stdout: "stderr1".to_string(),
            stderr: "stderr1".to_string(),
            success: true,
        };
        let script_result2 = TestResult::ScriptResult {
            name: "script2".to_string(),
            exit_code: 0,
            script: "script1".to_string(),
            output: "output2".to_string(),
            stdout: "stderr2".to_string(),
            stderr: "stderr2".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&script_result1);
        state.add_result(&script_result2);
        assert_eq!(state.get_script_output("script1"), Some("output1"));
        assert_eq!(state.get_script_output("script2"), Some("output2"));
    }

    #[test]
    fn returns_none_when_script_output_does_not_exists() {
        let state = State::new();
        assert_eq!(state.get_script_output("does-not-exist"), None);
    }

    #[test]
    fn increments_verify_counter_when_verify_success() {
        let verify_result1 = TestResult::VerifyResult {
            script_name: "script1".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: true,
        };
        let verify_result2 = TestResult::VerifyResult {
            script_name: "script2".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: true,
        };
        let mut state = State::new();
        state.add_result(&verify_result1);
        assert_eq!(state.number_of_verifies(), 1);
        state.add_result(&verify_result2);
        assert_eq!(state.number_of_verifies(), 2);
    }

    #[test]
    fn does_not_fail_when_verify_was_successful() {
        let verify_result = TestResult::VerifyResult {
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
    fn does_not_fail_when_verify_was_successful_after_failure() {
        let verify_result_failure = TestResult::VerifyResult {
            script_name: "script1".to_string(),
            stream: "output".to_string(),
            expected: "abc".to_string(),
            got: "abc".to_string(),
            success: false,
        };
        let verify_result_success = TestResult::VerifyResult {
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
        let verify_result = TestResult::VerifyResult {
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
}
