use nom::{
    bytes::streaming::{tag, take_until},
    combinator::map,
    sequence::tuple,
};

use crate::types::{ExitCode, FilePath, OutputExpectation, ScriptName, Source, Stream, TargetOs};

use super::error::{Error, Result};
use super::function_string_parser;
use super::function_string_parser::Function;

#[derive(Debug, PartialEq)]
pub struct ScriptCodeBlock {
    pub script_name: Option<ScriptName>,
    pub expected_exit_code: Option<ExitCode>,
    pub expected_output: OutputExpectation,
}

#[derive(Debug, PartialEq)]
pub enum CodeBlockType {
    Script(ScriptCodeBlock),
    Verify(Source),
    CreateFile(FilePath),
    Skip(),
}

#[derive(Debug, PartialEq)]
pub struct CodeBlockInfo {
    pub language: String,
    pub code_block_type: CodeBlockType,
}

pub fn parse(input: &str) -> Result<CodeBlockInfo> {
    let split_on_comma = tuple((take_until(","), tag(","), function_string_parser::parse));
    let mut parse_codeblock_info = map(split_on_comma, |(language, _comma, func)| (language, func));

    match parse_codeblock_info(input) {
        Ok((_, (language, func))) => {
            to_code_block_type(&func).map(|code_block_type| CodeBlockInfo {
                language: language.to_string(),
                code_block_type,
            })
        }
        Err(nom_error) => Err(Error::ParserFailed(format!(
            "Failed parsing function from '{}' :: {}",
            input, nom_error
        ))),
    }
}

fn to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    match &f.name[..] {
        "script" => script_to_code_block_type(f),
        "verify" => verify_to_code_block_type(f),
        "file" => file_to_code_block_type(f),
        "skip" => Ok(skip_to_code_block_type(f)),
        _ => Err(Error::UnknownFunction(f.name.clone())),
    }
}

fn script_to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    let name = if f.has_argument("name") {
        Some(ScriptName(get_string_argument(f, "name")?))
    } else {
        None
    };
    let expected_exit_code = if f.has_argument("expected_exit_code") {
        Some(ExitCode(get_integer_argument(f, "expected_exit_code")?))
    } else {
        None
    };
    let expected_output = get_token_argument(f, "expected_output")
        .or_else(|_| Ok("any".to_string()))
        .and_then(|s| to_expected_output(&s))?;
    Ok(CodeBlockType::Script(ScriptCodeBlock {
        script_name: name,
        expected_exit_code,
        expected_output,
    }))
}

fn to_expected_output(s: &str) -> Result<OutputExpectation> {
    match s {
        "any" => Ok(OutputExpectation::Any),
        "stdout" => Ok(OutputExpectation::StdOut),
        "stderr" => Ok(OutputExpectation::StdErr),
        "none" => Ok(OutputExpectation::None),
        _ => Err(Error::InvalidArgumentValue {
            function: "script".to_string(),
            argument: "expected_output".to_string(),
            expected: "any, stdout, stderr or none".to_string(),
            got: s.to_string(),
        }),
    }
}

fn file_to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    let path = get_string_argument(f, "path")?;
    Ok(CodeBlockType::CreateFile(FilePath(path)))
}

const fn skip_to_code_block_type(_f: &Function) -> CodeBlockType {
    CodeBlockType::Skip()
}

fn verify_to_code_block_type(f: &Function) -> Result<CodeBlockType> {
    let name = if f.has_argument("script_name") {
        Some(ScriptName(get_string_argument(f, "script_name")?))
    } else {
        None
    };
    let stream_name = if f.has_argument("stream") {
        get_token_argument(f, "stream")?
    } else {
        "stdout".to_string()
    };
    let target_os = if f.has_argument("target_os") {
        Some(TargetOs(get_string_argument(f, "target_os")?))
    } else {
        None
    };
    let stream = to_stream(&stream_name).ok_or_else(|| Error::InvalidArgumentValue {
        function: f.name.to_string(),
        argument: "stream".to_string(),
        got: stream_name.to_string(),
        expected: "output, stdout or stderr".to_string(),
    })?;
    Ok(CodeBlockType::Verify(Source {
        name,
        stream,
        target_os,
    }))
}

fn to_stream(stream_name: &str) -> Option<Stream> {
    match stream_name {
        "stdout" => Some(Stream::StdOut),
        "stderr" => Some(Stream::StdErr),
        _ => None,
    }
}

fn get_integer_argument(f: &Function, name: &str) -> Result<i32> {
    f.get_integer_argument(name)
        .map_err(Error::FunctionStringParser)
}

fn get_string_argument(f: &Function, name: &str) -> Result<String> {
    f.get_string_argument(name)
        .map_err(Error::FunctionStringParser)
}

fn get_token_argument(f: &Function, name: &str) -> Result<String> {
    f.get_token_argument(name)
        .map_err(Error::FunctionStringParser)
}

#[cfg(test)]
mod tests {
    use super::{
        parse, CodeBlockInfo, CodeBlockType, Error, ExitCode, FilePath, OutputExpectation,
        ScriptCodeBlock, ScriptName, Source, Stream,
    };

    mod parse {
        use super::{
            parse, CodeBlockInfo, CodeBlockType, Error, ExitCode, FilePath, OutputExpectation,
            ScriptCodeBlock, ScriptName, Source, Stream,
        };

        mod script {
            use super::{
                parse, CodeBlockInfo, CodeBlockType, ExitCode, OutputExpectation, ScriptCodeBlock,
                ScriptName,
            };

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
                        })
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
                        })
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
                        })
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
                        })
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
                        })
                    })
                );
            }
        }

        mod verify {
            use crate::types::TargetOs;

            use super::{parse, CodeBlockInfo, CodeBlockType, Error, ScriptName, Source, Stream};

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
                        })
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
                        })
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
                        })
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
                        })
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
                        })
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
                        })
                    })
                );
            }
        }

        mod file {
            use crate::parsers::function_string_parser;

            use super::{parse, CodeBlockInfo, CodeBlockType, Error, FilePath};

            #[test]
            fn succeeds_when_function_is_file() {
                let result = parse("text,file(path=\"example.txt\")");
                assert_eq!(
                    result,
                    Ok(CodeBlockInfo {
                        language: "text".to_string(),
                        code_block_type: CodeBlockType::CreateFile(FilePath(
                            "example.txt".to_string()
                        ))
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
                        code_block_type: CodeBlockType::Skip()
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
