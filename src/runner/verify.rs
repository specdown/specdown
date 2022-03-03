use crate::ansi::strip_ansi_escape_chars;
use crate::results::{ActionResult, VerifyResult};
use crate::runner::state::ScriptOutput;
use crate::types::{Source, Stream, VerifyAction};

use super::Error;

pub fn run(action: &VerifyAction, script_output: &dyn ScriptOutput) -> Result<ActionResult, Error> {
    let Source { name, stream } = action.source.clone();

    let result = name
        .as_ref()
        .map(|script_name| script_output.get_result(&String::from(script_name)))
        .or_else(|| Some(script_output.get_last_result()))
        .flatten();

    let script_name = result.and_then(|r| r.action.script_name.clone());

    result
        .map(|result| match stream {
            Stream::StdErr => result.stderr.clone(),
            Stream::StdOut => result.stdout.clone(),
        })
        .map(|got| {
            ActionResult::Verify(VerifyResult {
                action: action.with_script_name(script_name),
                got: strip_ansi_escape_chars(&got),
            })
        })
        .ok_or(Error::ScriptOutputMissing {
            missing_script_name: name.map_or("<unnamed>".to_string(), String::from),
        })
}

#[cfg(test)]
mod tests {
    use super::{run, ActionResult, Error, ScriptOutput};
    use crate::results::ScriptResult;
    use crate::types::{OutputExpectation, ScriptAction, ScriptCode, ScriptName};

    struct MockScriptOutput {
        result: Option<ScriptResult>,
    }

    impl MockScriptOutput {
        const fn without_result() -> Self {
            Self { result: None }
        }

        fn with_result(name: &str, stdout: &str, stderr: &str) -> Self {
            Self {
                result: Some(ScriptResult {
                    action: ScriptAction {
                        script_name: Some(ScriptName(name.to_string())),
                        script_code: ScriptCode("".to_string()),
                        expected_exit_code: None,
                        expected_output: OutputExpectation::Any,
                    },
                    exit_code: None,
                    stdout: stdout.to_string(),
                    stderr: stderr.to_string(),
                }),
            }
        }

        fn with_unnamed_result(stdout: &str, stderr: &str) -> Self {
            Self {
                result: Some(ScriptResult {
                    action: ScriptAction {
                        script_name: None,
                        script_code: ScriptCode("".to_string()),
                        expected_exit_code: None,
                        expected_output: OutputExpectation::Any,
                    },
                    exit_code: None,
                    stdout: stdout.to_string(),
                    stderr: stderr.to_string(),
                }),
            }
        }
    }

    impl ScriptOutput for MockScriptOutput {
        fn get_result(&self, name: &str) -> Option<&ScriptResult> {
            let unwrapped_result = self
                .result
                .as_ref()
                .expect("Result must be set to use get_result()");
            if Some(ScriptName(name.to_string())) == unwrapped_result.action.script_name {
                Some(unwrapped_result)
            } else {
                None
            }
        }

        fn get_last_result(&self) -> Option<&ScriptResult> {
            self.result.as_ref()
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
            let script_output = MockScriptOutput::with_unnamed_result("hello world", "");

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
        fn returns_result_for_stdout_verification_with_unnamed_verification() {
            let source = Source {
                name: None,
                stream: Stream::StdOut,
            };
            let verify_value = VerifyValue("hello world".to_string());
            let script_output = MockScriptOutput::with_result("example_script", "hello world", "");

            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Ok(ActionResult::Verify(VerifyResult {
                    action: action.with_script_name(Some(ScriptName("example_script".to_string()))),
                    got: "hello world".to_string(),
                }))
            );
        }

        #[test]
        fn returns_result_for_stderr_verification_with_unnamed_verification() {
            let source = Source {
                name: None,
                stream: Stream::StdErr,
            };
            let verify_value = VerifyValue("hello world".to_string());
            let script_output = MockScriptOutput::with_result("example_script", "", "hello world");

            let action = VerifyAction {
                source,
                expected_value: verify_value,
            };

            assert_eq!(
                run(&action, &script_output),
                Ok(ActionResult::Verify(VerifyResult {
                    action: action.with_script_name(Some(ScriptName("example_script".to_string()))),
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
            let script_output = MockScriptOutput::with_result("example_script", "hello world", "");

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
            let script_output =
                MockScriptOutput::with_result("my_script", "hello world", "error message");

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
            let script_output = MockScriptOutput::with_result("existing_script", "", "");
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
            let script_output = MockScriptOutput::without_result();
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
            let script_output =
                MockScriptOutput::with_result("colour_script", "\x1b[31mThis is coloured", "");
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
