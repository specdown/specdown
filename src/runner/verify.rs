use crate::ansi::strip_ansi_escape_chars;
use crate::results::{ActionResult, VerifyResult};
use crate::runner::state::ScriptOutput;
use crate::types::{Source, Stream, VerifyAction};

use super::Error;

pub fn run(action: &VerifyAction, script_output: &dyn ScriptOutput) -> Result<ActionResult, Error> {
    let Source { name, stream } = action.source.clone();

    let got_raw = match (name.clone(), stream) {
        (Some(script_name), Stream::StdErr) => script_output.get_stderr(&String::from(script_name)),
        (Some(script_name), Stream::StdOut) => script_output.get_stdout(&String::from(script_name)),
        (None, Stream::StdErr) => script_output.get_last_stderr(),
        (None, Stream::StdOut) => script_output.get_last_stdout(),
    };

    match got_raw {
        None => Err(Error::ScriptOutputMissing {
            missing_script_name: name.map_or("<unnamed>".to_string(), String::from),
        }),
        Some(got_raw) => {
            let got = strip_ansi_escape_chars(&got_raw);

            let result = ActionResult::Verify(VerifyResult {
                action: (*action).clone(),
                got,
            });

            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{run, ActionResult, Error, ScriptOutput};

    struct MockScriptOutput {
        script_name: String,
        stdout: String,
        stderr: String,
        last_stdout: Option<String>,
        last_stderr: Option<String>,
    }

    impl ScriptOutput for MockScriptOutput {
        fn get_stdout(&self, name: &str) -> Option<String> {
            if name == self.script_name {
                Some(self.stdout.clone())
            } else {
                None
            }
        }

        fn get_stderr(&self, name: &str) -> Option<String> {
            if name == self.script_name {
                Some(self.stderr.clone())
            } else {
                None
            }
        }

        fn get_last_stdout(&self) -> Option<String> {
            self.last_stdout.clone()
        }

        fn get_last_stderr(&self) -> Option<String> {
            self.last_stderr.clone()
        }
    }

    mod test {
        use crate::results::VerifyResult;
        use crate::types::{ScriptName, Source, Stream, VerifyAction, VerifyValue};

        use super::{run, ActionResult, Error, MockScriptOutput};

        #[test]
        fn returns_result_for_stdout_verification_with_unnamed_script() {
            let source = Source {
                name: None,
                stream: Stream::StdOut,
            };
            let verify_value = VerifyValue("hello world".to_string());
            let script_output = MockScriptOutput {
                script_name: "example_script".to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
                last_stdout: Some("hello world".to_string()),
                last_stderr: Some("".to_string()),
            };

            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Ok(ActionResult::Verify(VerifyResult {
                    action,
                    got: "hello world".to_string(),
                }))
            );
        }

        #[test]
        fn returns_result_for_stderr_verification_with_unnamed_script() {
            let source = Source {
                name: None,
                stream: Stream::StdErr,
            };
            let verify_value = VerifyValue("hello world".to_string());
            let script_output = MockScriptOutput {
                script_name: "example_script".to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
                last_stdout: Some("".to_string()),
                last_stderr: Some("hello world".to_string()),
            };

            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Ok(ActionResult::Verify(VerifyResult {
                    action,
                    got: "hello world".to_string(),
                }))
            );
        }

        #[test]
        fn returns_result_for_stdout_verification() {
            let source = Source {
                name: Some(ScriptName("example_script".to_string())),
                stream: Stream::StdOut,
            };
            let verify_value = VerifyValue("hello world".to_string());
            let script_output = MockScriptOutput {
                script_name: "example_script".to_string(),
                stdout: "hello world".to_string(),
                stderr: "".to_string(),
                last_stdout: None,
                last_stderr: None,
            };

            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Ok(ActionResult::Verify(VerifyResult {
                    action,
                    got: "hello world".to_string(),
                }))
            );
        }

        #[test]
        fn returns_result_for_stderr_verification() {
            let source = Source {
                name: Some(ScriptName("my_script".to_string())),
                stream: Stream::StdErr,
            };
            let verify_value = VerifyValue("error message".to_string());
            let script_output = MockScriptOutput {
                script_name: "my_script".to_string(),
                stdout: "hello world".to_string(),
                stderr: "error message".to_string(),
                last_stdout: None,
                last_stderr: None,
            };
            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Ok(ActionResult::Verify(VerifyResult {
                    action,
                    got: "error message".to_string(),
                }))
            );
        }

        #[test]
        fn returns_error_when_script_output_does_not_exit() {
            let source = Source {
                name: Some(ScriptName("missing_script".to_string())),
                stream: Stream::StdErr,
            };
            let verify_value = VerifyValue("error message".to_string());
            let script_output = MockScriptOutput {
                script_name: "existing_script".to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
                last_stdout: None,
                last_stderr: None,
            };
            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Err(Error::ScriptOutputMissing {
                    missing_script_name: "missing_script".to_string()
                })
            );
        }
        #[test]
        fn returns_error_when_unnamed_script_output_does_not_exit() {
            let source = Source {
                name: None,
                stream: Stream::StdErr,
            };
            let verify_value = VerifyValue("error message".to_string());
            let script_output = MockScriptOutput {
                script_name: "existing_script".to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
                last_stdout: None,
                last_stderr: None,
            };
            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Err(Error::ScriptOutputMissing {
                    missing_script_name: "<unnamed>".to_string()
                })
            );
        }

        #[test]
        fn ignore_ansi_escape_characters_in_output_and_verify_value() {
            let source = Source {
                name: Some(ScriptName("colour_script".to_string())),
                stream: Stream::StdOut,
            };
            let verify_value = VerifyValue("\x1b[34mThis is coloured".to_string());
            let script_output = MockScriptOutput {
                script_name: "colour_script".to_string(),
                stdout: "\x1b[31mThis is coloured".to_string(),
                stderr: "".to_string(),
                last_stdout: None,
                last_stderr: None,
            };
            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Ok(ActionResult::Verify(VerifyResult {
                    action,
                    got: "This is coloured".to_string(),
                }))
            );
        }
    }
}
