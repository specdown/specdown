use std::collections::HashMap;

use nom::{
    bytes::streaming::{tag, take_until},
    combinator::map,
    sequence::tuple,
};

use crate::parser::function_string;
use crate::types::{ScriptName, Source, Stream};

#[derive(Debug, PartialEq)]
pub enum BlockQuoteTypes {
    Script(ScriptName),
    Verify(Source),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ParserError(String),
    UnknownFunction(String),
    MissingArgumentError(String, String),
    IncorrectArgumentType { expected: String, got: String },
    InvalidArgumentValue { got: String, expected: String },
}

pub fn parse(input: &str) -> Result<BlockQuoteTypes, Error> {
    let p = tuple((take_until(","), tag(","), function_string::parse));
    let p = map(p, |(_language, _comma, func)| func);

    match p(input) {
        Ok((_, func)) => to_blockquote_type(&func),
        Err(_nom_error) => Err(Error::ParserError(String::from(_nom_error.to_string()))),
    }
}

fn to_blockquote_type(f: &function_string::Function) -> Result<BlockQuoteTypes, Error> {
    match f.name {
        "script" => {
            let name = get_string_argument(&f, "name")?;
            Ok(BlockQuoteTypes::Script(ScriptName(name)))
        }
        "verify" => {
            let script_name = get_string_argument(&f, "script_name")?;
            let stream = to_stream(&get_token_argument(&f, "stream")?)?;
            Ok(BlockQuoteTypes::Verify(Source {
                name: ScriptName(script_name),
                stream: stream,
            }))
        }
        _ => Err(Error::UnknownFunction(String::from(f.name))),
    }
}

fn to_stream(stream_name: &str) -> Result<Stream, Error> {
    match stream_name {
        "output" => Ok(Stream::Output),
        "stdout" => Ok(Stream::StdOut),
        "stderr" => Ok(Stream::StdErr),
        _ => Err(Error::InvalidArgumentValue {
            got: stream_name.to_string(),
            expected: "output, stdout or stderr".to_string(),
        }),
    }
}

fn get_string_argument(f: &function_string::Function, name: &str) -> Result<String, Error> {
    let arg = f.arguments.get(name).ok_or(Error::MissingArgumentError(
        String::from(f.name),
        name.to_string(),
    ))?;

    match arg {
        function_string::ArgumentValue::String(s) => Ok(String::from(*s)),
        function_string::ArgumentValue::Token(_) => Err(Error::IncorrectArgumentType {
            expected: "string".to_string(),
            got: "token".to_string(),
        }),
    }
}

fn get_token_argument(f: &function_string::Function, name: &str) -> Result<String, Error> {
    let arg = f.arguments.get(name).ok_or(Error::MissingArgumentError(
        String::from(f.name),
        name.to_string(),
    ))?;

    match arg {
        function_string::ArgumentValue::Token(t) => Ok(String::from(*t)),
        function_string::ArgumentValue::String(_) => Err(Error::IncorrectArgumentType {
            expected: "token".to_string(),
            got: "string".to_string(),
        }),
    }
}

mod tests {
    use super::*;

    mod parse {
        use super::*;

        mod script {
            use super::*;

            #[test]
            fn succeeds_when_function_is_script() {
                let result = parse("shell,script(name=\"example-script\")");
                assert_eq!(
                    result,
                    Ok(BlockQuoteTypes::Script(ScriptName(
                        "example-script".to_string()
                    )))
                )
            }

            #[test]
            fn fails_when_name_is_missing() {
                let result = parse("shell,script()");
                assert_eq!(
                    result,
                    Err(Error::MissingArgumentError(
                        "script".to_string(),
                        "name".to_string()
                    ))
                )
            }
        }

        mod verify {
            use super::*;

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_output() {
                let result = parse(",verify(script_name=\"example-script\", stream=output)");
                assert_eq!(
                    result,
                    Ok(BlockQuoteTypes::Verify(Source {
                        name: ScriptName("example-script".to_string()),
                        stream: Stream::Output
                    }))
                )
            }

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_stdout() {
                let result = parse(",verify(script_name=\"example-script\", stream=stdout)");
                assert_eq!(
                    result,
                    Ok(BlockQuoteTypes::Verify(Source {
                        name: ScriptName("example-script".to_string()),
                        stream: Stream::StdOut
                    }))
                )
            }

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_stderr() {
                let result = parse(",verify(script_name=\"example-script\", stream=stderr)");
                assert_eq!(
                    result,
                    Ok(BlockQuoteTypes::Verify(Source {
                        name: ScriptName("example-script".to_string()),
                        stream: Stream::StdErr
                    }))
                )
            }

            #[test]
            fn fails_when_function_is_verify_and_stream_is_unknown() {
                let result = parse(",verify(script_name=\"example-script\", stream=unknown)");
                assert_eq!(
                    result,
                    Err(Error::InvalidArgumentValue {
                        got: "unknown".to_string(),
                        expected: "output, stdout or stderr".to_string()
                    })
                )
            }

            #[test]
            fn fails_when_script_name_is_missing() {
                let result = parse("shell,verify(stream=stderr)");
                assert_eq!(
                    result,
                    Err(Error::MissingArgumentError(
                        "verify".to_string(),
                        "script_name".to_string()
                    ))
                )
            }

            #[test]
            fn fails_when_stream_is_missing() {
                let result = parse("shell,verify(script_name=\"the-script\")");
                assert_eq!(
                    result,
                    Err(Error::MissingArgumentError(
                        "verify".to_string(),
                        "stream".to_string()
                    ))
                )
            }
        }
    }
}
