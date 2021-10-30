use nom::{
    bytes::streaming::{tag, take_until},
    combinator::map,
    sequence::tuple,
};

use super::error::{Error, Result};
use super::function;
use super::function_string;
use crate::types::{ExitCode, FilePath, OutputExpectation, ScriptName, Source, Stream};

#[derive(Debug, PartialEq)]
pub enum CodeBlockType {
    Script(Option<ScriptName>, Option<ExitCode>, OutputExpectation),
    Verify(Source),
    CreateFile(FilePath),
    Skip(),
}

pub fn parse(input: &str) -> Result<(&str, CodeBlockType)> {
    let split_on_comma = tuple((take_until(","), tag(","), function_string::parse));
    let mut parse_codeblock_info = map(split_on_comma, |(language, _comma, func)| (language, func));

    match parse_codeblock_info(input) {
        Ok((_, (language, func))) => {
            to_code_block_type(&func).map(|code_block_type| (language, code_block_type))
        }
        Err(nom_error) => Err(Error::ParserFailed(format!(
            "Failed parsing function from '{}' :: {}",
            input, nom_error
        ))),
    }
}

fn to_code_block_type(f: &function::Function) -> Result<CodeBlockType> {
    match &f.name[..] {
        "script" => script_to_code_block_type(f),
        "verify" => verify_to_code_block_type(f),
        "file" => file_to_code_block_type(f),
        "skip" => Ok(skip_to_code_block_type(f)),
        _ => Err(Error::UnknownFunction(f.name.clone())),
    }
}

fn script_to_code_block_type(f: &function::Function) -> Result<CodeBlockType> {
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
    Ok(CodeBlockType::Script(
        name,
        expected_exit_code,
        expected_output,
    ))
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

fn file_to_code_block_type(f: &function::Function) -> Result<CodeBlockType> {
    let path = get_string_argument(f, "path")?;
    Ok(CodeBlockType::CreateFile(FilePath(path)))
}

fn skip_to_code_block_type(_f: &function::Function) -> CodeBlockType {
    CodeBlockType::Skip()
}

fn verify_to_code_block_type(f: &function::Function) -> Result<CodeBlockType> {
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
    let stream = to_stream(&stream_name).ok_or_else(|| Error::InvalidArgumentValue {
        function: f.name.to_string(),
        argument: "stream".to_string(),
        got: stream_name.to_string(),
        expected: "output, stdout or stderr".to_string(),
    })?;
    Ok(CodeBlockType::Verify(Source { name, stream }))
}

fn to_stream(stream_name: &str) -> Option<Stream> {
    match stream_name {
        "stdout" => Some(Stream::StdOut),
        "stderr" => Some(Stream::StdErr),
        _ => None,
    }
}

fn get_integer_argument(f: &function::Function, name: &str) -> Result<i32> {
    f.get_integer_argument(name)
}

fn get_string_argument(f: &function::Function, name: &str) -> Result<String> {
    f.get_string_argument(name)
}

fn get_token_argument(f: &function::Function, name: &str) -> Result<String> {
    f.get_token_argument(name)
}

#[cfg(test)]
mod tests {
    use super::{
        parse, CodeBlockType, Error, ExitCode, FilePath, OutputExpectation, ScriptName, Source,
        Stream,
    };

    mod parse {
        use super::{
            parse, CodeBlockType, Error, ExitCode, FilePath, OutputExpectation, ScriptName, Source,
            Stream,
        };

        mod script {
            use super::{parse, CodeBlockType, ExitCode, OutputExpectation, ScriptName};

            #[test]
            fn succeeds_when_function_is_script_with_a_name() {
                let result = parse("shell,script(name=\"example-script\")");
                assert_eq!(
                    result,
                    Ok((
                        "shell",
                        CodeBlockType::Script(
                            Some(ScriptName("example-script".to_string())),
                            None,
                            OutputExpectation::Any
                        )
                    ))
                );
            }

            #[test]
            fn succeeds_when_function_is_script_without_a_name() {
                let result = parse("shell,script()");
                assert_eq!(
                    result,
                    Ok((
                        "shell",
                        CodeBlockType::Script(None, None, OutputExpectation::Any)
                    ))
                );
            }

            #[test]
            fn succeeds_when_function_is_script_with_expected_exit_code() {
                let result = parse("shell,script(name=\"example-script\", expected_exit_code=2)");
                assert_eq!(
                    result,
                    Ok((
                        "shell",
                        CodeBlockType::Script(
                            Some(ScriptName("example-script".to_string())),
                            Some(ExitCode(2)),
                            OutputExpectation::Any,
                        )
                    ))
                );
            }

            #[test]
            fn succeeds_when_function_is_script_with_expected_output_set_to_any() {
                let result = parse("shell,script(name=\"example-script\", expected_output=any)");
                assert_eq!(
                    result,
                    Ok((
                        "shell",
                        CodeBlockType::Script(
                            Some(ScriptName("example-script".to_string())),
                            None,
                            OutputExpectation::Any,
                        )
                    ))
                );
            }

            #[test]
            fn succeeds_when_function_is_script_with_expected_output_set_to_stdout() {
                let result = parse("shell,script(name=\"example-script\", expected_output=stdout)");
                assert_eq!(
                    result,
                    Ok((
                        "shell",
                        CodeBlockType::Script(
                            Some(ScriptName("example-script".to_string())),
                            None,
                            OutputExpectation::StdOut,
                        )
                    ))
                );
            }
        }

        mod verify {
            use super::{parse, CodeBlockType, Error, ScriptName, Source, Stream};

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_stdout() {
                let result = parse(",verify(script_name=\"example-script\", stream=stdout)");
                assert_eq!(
                    result,
                    Ok((
                        "",
                        CodeBlockType::Verify(Source {
                            name: Some(ScriptName("example-script".to_string())),
                            stream: Stream::StdOut
                        })
                    ))
                );
            }

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_stderr() {
                let result = parse(",verify(script_name=\"example-script\", stream=stderr)");
                assert_eq!(
                    result,
                    Ok((
                        "",
                        CodeBlockType::Verify(Source {
                            name: Some(ScriptName("example-script".to_string())),
                            stream: Stream::StdErr
                        })
                    ))
                );
            }

            #[test]
            fn succeeds_and_defaults_to_stdout_when_the_stream_is_missing() {
                let result = parse(",verify(script_name=\"the-script\")");
                assert_eq!(
                    result,
                    Ok((
                        "",
                        CodeBlockType::Verify(Source {
                            name: Some(ScriptName("the-script".to_string())),
                            stream: Stream::StdOut
                        })
                    ))
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
                    Ok((
                        "text",
                        CodeBlockType::Verify(Source {
                            name: None,
                            stream: Stream::StdErr
                        })
                    ))
                );
            }
        }

        mod file {
            use super::{parse, CodeBlockType, Error, FilePath};

            #[test]
            fn succeeds_when_function_is_file() {
                let result = parse("text,file(path=\"example.txt\")");
                assert_eq!(
                    result,
                    Ok((
                        "text",
                        CodeBlockType::CreateFile(FilePath("example.txt".to_string()))
                    ))
                );
            }

            #[test]
            fn fails_when_path_is_missing() {
                let result = parse("text,file()");
                assert_eq!(
                    result,
                    Err(Error::MissingArgument {
                        function: "file".to_string(),
                        argument: "path".to_string()
                    })
                );
            }
        }

        mod skip {
            use super::{parse, CodeBlockType, Error};

            #[test]
            fn succeeds_when_function_is_skip() {
                let result = parse("text,skip()");
                assert_eq!(result, Ok(("text", CodeBlockType::Skip())));
            }

            #[test]
            fn fails_when_path_is_missing() {
                let result = parse("text,file()");
                assert_eq!(
                    result,
                    Err(Error::MissingArgument {
                        function: "file".to_string(),
                        argument: "path".to_string()
                    })
                );
            }
        }
    }
}
