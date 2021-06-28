use crate::results::action_result::ActionResult;
use crate::runner::state::ScriptOutput;
use crate::types::{ScriptName, Source, Stream, VerifyValue};

use super::error::Error;

pub fn run(
    source: &Source,
    value: &VerifyValue,
    script_output: &dyn ScriptOutput,
) -> Result<ActionResult, Error> {
    let Source {
        name: ScriptName(script_name),
        stream,
    } = source;
    let VerifyValue(value_string) = value;

    let got_raw = match stream {
        Stream::StdOut => script_output.get_stdout(script_name),
        Stream::StdErr => script_output.get_stderr(script_name),
    };

    match got_raw {
        None => Err(Error::ScriptOutputMissing {
            missing_script_name: script_name.to_string(),
        }),
        Some(got_raw) => {
            let expected = strip_ansi_escape_chars(value_string);
            let got = strip_ansi_escape_chars(got_raw);
            let success = expected == got;

            let result = ActionResult::Verify {
                script_name: script_name.to_string(),
                stream: stream_to_string(stream).into(),
                expected,
                got,
                success,
            };

            Ok(result)
        }
    }
}

fn stream_to_string(stream: &Stream) -> &str {
    match stream {
        Stream::StdOut => "stdout",
        Stream::StdErr => "stderr",
    }
}

fn strip_ansi_escape_chars(string: &str) -> String {
    strip_ansi_escapes::strip(string)
        .expect("ANSI code to be stripped from got")
        .iter()
        .map(|&c| c as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{run, ActionResult, Error, ScriptOutput};

    struct MockScriptOutput {
        script_name: String,
        stdout: String,
        stderr: String,
    }

    impl ScriptOutput for MockScriptOutput {
        fn get_stdout(&self, name: &str) -> Option<&str> {
            if name == self.script_name {
                Some(self.stdout.as_ref())
            } else {
                None
            }
        }

        fn get_stderr(&self, name: &str) -> Option<&str> {
            if name == self.script_name {
                Some(self.stderr.as_ref())
            } else {
                None
            }
        }
    }

    mod test {
        use crate::types::{ScriptName, Source, Stream, VerifyValue};

        use super::{run, ActionResult, Error, MockScriptOutput};

        #[test]
        fn returns_result_for_successful_stdout_verification() {
            let source = Source {
                name: ScriptName("example_script".to_string()),
                stream: Stream::StdOut,
            };
            let verify_value = VerifyValue("hello world".to_string());
            let script_output = MockScriptOutput {
                script_name: "example_script".to_string(),
                stdout: "hello world".to_string(),
                stderr: "".to_string(),
            };

            assert_eq!(
                run(&source, &verify_value, &script_output),
                Ok(ActionResult::Verify {
                    script_name: "example_script".to_string(),
                    stream: "stdout".to_string(),
                    expected: "hello world".to_string(),
                    got: "hello world".to_string(),
                    success: true,
                })
            );
        }

        #[test]
        fn returns_result_for_successful_error_verification() {
            let source = Source {
                name: ScriptName("my_script".to_string()),
                stream: Stream::StdErr,
            };
            let verify_value = VerifyValue("error message".to_string());
            let script_output = MockScriptOutput {
                script_name: "my_script".to_string(),
                stdout: "hello world".to_string(),
                stderr: "error message".to_string(),
            };

            assert_eq!(
                run(&source, &verify_value, &script_output),
                Ok(ActionResult::Verify {
                    script_name: "my_script".to_string(),
                    stream: "stderr".to_string(),
                    expected: "error message".to_string(),
                    got: "error message".to_string(),
                    success: true,
                })
            );
        }

        #[test]
        fn returns_result_for_failed_stdout_verification() {
            let source = Source {
                name: ScriptName("test_script".to_string()),
                stream: Stream::StdOut,
            };
            let verify_value = VerifyValue("hello moon".to_string());
            let script_output = MockScriptOutput {
                script_name: "test_script".to_string(),
                stdout: "hello mars".to_string(),
                stderr: "".to_string(),
            };

            assert_eq!(
                run(&source, &verify_value, &script_output),
                Ok(ActionResult::Verify {
                    script_name: "test_script".to_string(),
                    stream: "stdout".to_string(),
                    expected: "hello moon".to_string(),
                    got: "hello mars".to_string(),
                    success: false,
                })
            );
        }

        #[test]
        fn returns_result_for_failed_error_verification() {
            let source = Source {
                name: ScriptName("the_script".to_string()),
                stream: Stream::StdErr,
            };
            let verify_value = VerifyValue("error message".to_string());
            let script_output = MockScriptOutput {
                script_name: "the_script".to_string(),
                stdout: "hello world".to_string(),
                stderr: "not error message".to_string(),
            };

            assert_eq!(
                run(&source, &verify_value, &script_output),
                Ok(ActionResult::Verify {
                    script_name: "the_script".to_string(),
                    stream: "stderr".to_string(),
                    expected: "error message".to_string(),
                    got: "not error message".to_string(),
                    success: false,
                })
            );
        }

        #[test]
        fn returns_error_when_script_output_does_not_exit() {
            let source = Source {
                name: ScriptName("missing_script".to_string()),
                stream: Stream::StdErr,
            };
            let verify_value = VerifyValue("error message".to_string());
            let script_output = MockScriptOutput {
                script_name: "existing_script".to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
            };

            assert_eq!(
                run(&source, &verify_value, &script_output),
                Err(Error::ScriptOutputMissing {
                    missing_script_name: "missing_script".to_string()
                })
            );
        }

        #[test]
        fn ignore_ansi_escape_characters_in_output_and_verify_value() {
            let source = Source {
                name: ScriptName("colour_script".to_string()),
                stream: Stream::StdOut,
            };
            let verify_value = VerifyValue("\x1b[34mThis is coloured".to_string());
            let script_output = MockScriptOutput {
                script_name: "colour_script".to_string(),
                stdout: "\x1b[31mThis is coloured".to_string(),
                stderr: "".to_string(),
            };

            assert_eq!(
                run(&source, &verify_value, &script_output),
                Ok(ActionResult::Verify {
                    script_name: "colour_script".to_string(),
                    stream: "stdout".to_string(),
                    expected: "This is coloured".to_string(),
                    got: "This is coloured".to_string(),
                    success: true,
                })
            );
        }
    }
}
