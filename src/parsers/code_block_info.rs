use crate::parsers::code_block_type;
use crate::parsers::code_block_type::CodeBlockType;
use nom::bytes::streaming::{tag, take_until};
use nom::combinator::{map, map_res};
use nom::error::ParseError;
use nom::sequence::separated_pair;
use nom::{Compare, Err, FindSubstring, IResult, InputLength, InputTake, Parser};

use super::error::{Error, Result};
use super::function_string_parser;

#[derive(Debug, PartialEq)]
pub struct CodeBlockInfo<Extra> {
    pub language: String,
    pub code_block_type: Extra,
}

pub fn parse(input: &str) -> Result<CodeBlockInfo<CodeBlockType>> {
    match parser(input) {
        Ok((_, result)) => Ok(result),
        Err(err) => match err {
            Err::Incomplete(_) => panic!("code_block_info parser returned an Incomplete error"),
            Err::Error(e) | Err::Failure(e) => Err(e),
        },
    }
}

fn parser(input: &str) -> IResult<&str, CodeBlockInfo<CodeBlockType>, Error> {
    let code_block_type = map_res(
        function_string_parser::parse,
        code_block_type::from_function,
    );
    let parse_code_block_info = code_block_info(code_block_type);

    map(parse_code_block_info, |(language, code_block_type)| {
        CodeBlockInfo {
            language: language.to_string(),
            code_block_type,
        }
    })(input)
}

fn code_block_info<Input, Output, Error: ParseError<Input>, InnerParser>(
    extra_info_parser: InnerParser,
) -> impl FnMut(Input) -> IResult<Input, (Input, Output), Error>
where
    Input: InputTake + InputLength + Compare<&'static str> + FindSubstring<&'static str>,
    InnerParser: Parser<Input, Output, Error>,
{
    separated_pair(take_until(","), tag(","), extra_info_parser)
}

#[cfg(test)]
mod tests {
    use super::{parse, CodeBlockInfo, CodeBlockType, Error};

    mod parse {
        use super::{parse, CodeBlockInfo, CodeBlockType, Error};

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
                        code_block_type: CodeBlockType::Script(ScriptCodeBlock {
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
                        code_block_type: CodeBlockType::Script(ScriptCodeBlock {
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
                        code_block_type: CodeBlockType::Script(ScriptCodeBlock {
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
                        code_block_type: CodeBlockType::Script(ScriptCodeBlock {
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
                        code_block_type: CodeBlockType::Script(ScriptCodeBlock {
                            script_name: Some(ScriptName("example-script".to_string())),
                            expected_exit_code: None,
                            expected_output: OutputExpectation::StdOut,
                        }),
                    })
                );
            }
        }

        mod verify {
            use crate::types::{ScriptName, Source, Stream, TargetOs};

            use super::{parse, CodeBlockInfo, CodeBlockType, Error};

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_stdout() {
                let result = parse(",verify(script_name=\"example-script\", stream=stdout)");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "".to_string(),
                        code_block_type: CodeBlockType::Verify(Source {
                            name: Some(ScriptName("example-script".to_string())),
                            stream: Stream::StdOut,
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
                        code_block_type: CodeBlockType::Verify(Source {
                            name: Some(ScriptName("example-script".to_string())),
                            stream: Stream::StdErr,
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
                        code_block_type: CodeBlockType::Verify(Source {
                            name: Some(ScriptName("the-script".to_string())),
                            stream: Stream::StdOut,
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
                        code_block_type: CodeBlockType::Verify(Source {
                            name: Some(ScriptName("the-script".to_string())),
                            stream: Stream::StdOut,
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
                        code_block_type: CodeBlockType::Verify(Source {
                            name: Some(ScriptName("the-script".to_string())),
                            stream: Stream::StdOut,
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
                        code_block_type: CodeBlockType::Verify(Source {
                            name: None,
                            stream: Stream::StdErr,
                            target_os: None,
                        }),
                    })
                );
            }
        }

        mod file {
            use crate::parsers::function_string_parser;
            use crate::types::FilePath;

            use super::{parse, CodeBlockInfo, CodeBlockType, Error};

            #[test]
            fn succeeds_when_function_is_file() {
                let result = parse("text,file(path=\"example.txt\")");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "text".to_string(),
                        code_block_type: CodeBlockType::CreateFile(FilePath(
                            "example.txt".to_string()
                        )),
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
            use crate::parsers::function_string_parser;

            use super::{parse, CodeBlockInfo, CodeBlockType, Error};

            #[test]
            fn succeeds_when_function_is_skip() {
                let result = parse("text,skip()");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "text".to_string(),
                        code_block_type: CodeBlockType::Skip(),
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
