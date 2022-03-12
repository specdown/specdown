use crate::parsers::code_block_type;
use nom::Err;

use super::code_block_type::CodeBlockType;
use super::error::Result;
use super::markdown::code_block_info::CodeBlockInfo;
use crate::parsers::markdown::code_block_info;

pub fn parse(input: &str) -> Result<CodeBlockInfo<CodeBlockType>> {
    match code_block_info::parse(code_block_type::parse)(input) {
        Ok((_, result)) => Ok(result),
        Err(err) => match err {
            Err::Incomplete(_) => panic!("code_block_info parser returned an Incomplete error"),
            Err::Error(e) | Err::Failure(e) => Err(e),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, CodeBlockInfo, CodeBlockType};

    mod parse {
        use super::{parse, CodeBlockInfo, CodeBlockType};

        mod script {
            use super::{parse, CodeBlockInfo, CodeBlockType};
            use crate::parsers::code_block_type::ScriptCodeBlock;
            use crate::types::{ExitCode, OutputExpectation, ScriptName};

            #[test]
            fn succeeds_when_function_is_script_with_a_name() {
                let result = parse("shell,script(name=\"example-script\")");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "shell".to_string(),
                        extra: CodeBlockType::Script(ScriptCodeBlock {
                            script_name: Some(ScriptName("example-script".to_string())),
                            expected_exit_code: None,
                            expected_output: OutputExpectation::Any,
                        }),
                    })
                );
            }

            #[test]
            fn succeeds_when_function_is_script_without_a_name() {
                let result = parse("shell,script()");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "shell".to_string(),
                        extra: CodeBlockType::Script(ScriptCodeBlock {
                            script_name: None,
                            expected_exit_code: None,
                            expected_output: OutputExpectation::Any,
                        }),
                    })
                );
            }

            #[test]
            fn succeeds_when_function_is_script_with_expected_exit_code() {
                let result = parse("shell,script(name=\"example-script\", expected_exit_code=2)");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "shell".to_string(),
                        extra: CodeBlockType::Script(ScriptCodeBlock {
                            script_name: Some(ScriptName("example-script".to_string())),
                            expected_exit_code: Some(ExitCode(2)),
                            expected_output: OutputExpectation::Any,
                        }),
                    })
                );
            }

            #[test]
            fn succeeds_when_function_is_script_with_expected_output_set_to_any() {
                let result = parse("shell,script(name=\"example-script\", expected_output=any)");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "shell".to_string(),
                        extra: CodeBlockType::Script(ScriptCodeBlock {
                            script_name: Some(ScriptName("example-script".to_string())),
                            expected_exit_code: None,
                            expected_output: OutputExpectation::Any,
                        }),
                    })
                );
            }

            #[test]
            fn succeeds_when_function_is_script_with_expected_output_set_to_stdout() {
                let result = parse("shell,script(name=\"example-script\", expected_output=stdout)");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "shell".to_string(),
                        extra: CodeBlockType::Script(ScriptCodeBlock {
                            script_name: Some(ScriptName("example-script".to_string())),
                            expected_exit_code: None,
                            expected_output: OutputExpectation::StdOut,
                        }),
                    })
                );
            }
        }

        mod verify {
            use crate::parsers::code_block_type::VerifyCodeBlock;
            use crate::parsers::error::Error;
            use crate::types::{ScriptName, Source, Stream, TargetOs};

            use super::{parse, CodeBlockInfo, CodeBlockType};

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_stdout() {
                let result = parse(",verify(script_name=\"example-script\", stream=stdout)");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "".to_string(),
                        extra: CodeBlockType::Verify(VerifyCodeBlock {
                            source: Source {
                                name: Some(ScriptName("example-script".to_string())),
                                stream: Stream::StdOut,
                            },
                            target_os: None,
                        }),
                    })
                );
            }

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_stderr() {
                let result = parse(",verify(script_name=\"example-script\", stream=stderr)");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "".to_string(),
                        extra: CodeBlockType::Verify(VerifyCodeBlock {
                            source: Source {
                                name: Some(ScriptName("example-script".to_string())),
                                stream: Stream::StdErr,
                            },
                            target_os: None,
                        }),
                    })
                );
            }

            #[test]
            fn succeeds_and_defaults_to_stdout_when_the_stream_is_missing() {
                let result = parse(",verify(script_name=\"the-script\")");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "".to_string(),
                        extra: CodeBlockType::Verify(VerifyCodeBlock {
                            source: Source {
                                name: Some(ScriptName("the-script".to_string())),
                                stream: Stream::StdOut,
                            },
                            target_os: None,
                        }),
                    })
                );
            }

            #[test]
            fn target_os_defaults_to_none_when_function_is_verify_and_target_os_is_unset() {
                let result = parse(",verify(script_name=\"the-script\")");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "".to_string(),
                        extra: CodeBlockType::Verify(VerifyCodeBlock {
                            source: Source {
                                name: Some(ScriptName("the-script".to_string())),
                                stream: Stream::StdOut,
                            },
                            target_os: None,
                        }),
                    })
                );
            }

            #[test]
            fn target_os_can_be_set_when_function_is_verify_and_target_os_is_unset() {
                let result = parse(",verify(script_name=\"the-script\",target_os=\"some-os\")");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "".to_string(),
                        extra: CodeBlockType::Verify(VerifyCodeBlock {
                            source: Source {
                                name: Some(ScriptName("the-script".to_string())),
                                stream: Stream::StdOut,
                            },
                            target_os: Some(TargetOs("some-os".to_string())),
                        }),
                    })
                );
            }

            #[test]
            fn fails_when_function_is_verify_and_stream_is_unknown() {
                let result = parse(",verify(script_name=\"example-script\", stream=unknown)");
                assert_eq!(
                    result,
                    Err(Error::InvalidArgumentValue {
                        function: "verify".to_string(),
                        argument: "stream".to_string(),
                        expected: "output, stdout or stderr".to_string(),
                        got: "unknown".to_string(),
                    })
                );
            }

            #[test]
            fn succeeds_when_script_name_is_not_present() {
                let result = parse("text,verify(stream=stderr)");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "text".to_string(),
                        extra: CodeBlockType::Verify(VerifyCodeBlock {
                            source: Source {
                                name: None,
                                stream: Stream::StdErr,
                            },
                            target_os: None,
                        }),
                    })
                );
            }
        }

        mod file {
            use crate::parsers::error::Error;
            use crate::parsers::function_string_parser;
            use crate::types::FilePath;

            use super::{parse, CodeBlockInfo, CodeBlockType};

            #[test]
            fn succeeds_when_function_is_file() {
                let result = parse("text,file(path=\"example.txt\")");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "text".to_string(),
                        extra: CodeBlockType::CreateFile(FilePath("example.txt".to_string())),
                    })
                );
            }

            #[test]
            fn fails_when_path_is_missing() {
                let result = parse("text,file()");
                assert_eq!(
                    result,
                    Err(Error::FunctionStringParser(
                        function_string_parser::Error::MissingArgument {
                            function: "file".to_string(),
                            argument: "path".to_string(),
                        }
                    ))
                );
            }
        }

        mod skip {
            use crate::parsers::error::Error;
            use crate::parsers::function_string_parser;

            use super::{parse, CodeBlockInfo, CodeBlockType};

            #[test]
            fn succeeds_when_function_is_skip() {
                let result = parse("text,skip()");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "text".to_string(),
                        extra: CodeBlockType::Skip(),
                    })
                );
            }

            #[test]
            fn fails_when_path_is_missing() {
                let result = parse("text,file()");
                assert_eq!(
                    result,
                    Err(Error::FunctionStringParser(
                        function_string_parser::Error::MissingArgument {
                            function: "file".to_string(),
                            argument: "path".to_string(),
                        }
                    ))
                );
            }
        }
    }
}
