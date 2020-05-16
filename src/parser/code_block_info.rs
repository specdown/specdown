use nom::{
    bytes::streaming::{tag, take_until},
    combinator::map,
    sequence::tuple,
};

use crate::parser::error::{Error, Result};
use crate::parser::function_string;
use crate::types::{ScriptName, Source, Stream};

#[derive(Debug, PartialEq)]
pub enum CodeBlockType {
    Script(ScriptName),
    Verify(Source),
}

pub fn parse(input: &str) -> Result<CodeBlockType> {
    let p = tuple((take_until(","), tag(","), function_string::parse));
    let p = map(p, |(_language, _comma, func)| func);

    match p(input) {
        Ok((_, func)) => to_code_block_type(&func),
        Err(nom_error) => Err(Error::ParserFailed(nom_error.to_string())),
    }
}

fn to_code_block_type(f: &function_string::Function) -> Result<CodeBlockType> {
    match &f.name[..] {
        "script" => {
            let name = get_string_argument(&f, "name")?;
            Ok(CodeBlockType::Script(ScriptName(name)))
        }
        "verify" => {
            let name = ScriptName(get_string_argument(&f, "script_name")?);
            let stream_name = get_token_argument(&f, "stream")?;
            let stream = to_stream(&stream_name).ok_or_else(|| Error::InvalidArgumentValue {
                function: f.name.to_string(),
                argument: "stream".to_string(),
                got: stream_name.to_string(),
                expected: "output, stdout or stderr".to_string(),
            })?;
            Ok(CodeBlockType::Verify(Source { name, stream }))
        }
        _ => Err(Error::UnknownFunction(f.name.clone())),
    }
}

fn to_stream(stream_name: &str) -> Option<Stream> {
    match stream_name {
        "output" => Some(Stream::Output),
        "stdout" => Some(Stream::StdOut),
        "stderr" => Some(Stream::StdErr),
        _ => None,
    }
}

fn get_string_argument(f: &function_string::Function, name: &str) -> Result<String> {
    let arg = f
        .arguments
        .get(name)
        .ok_or_else(|| Error::MissingArgument {
            function: f.name.clone(),
            argument: name.to_string(),
        })?;

    match arg {
        function_string::ArgumentValue::String(s) => Ok(s.clone()),
        function_string::ArgumentValue::Token(_) => Err(Error::IncorrectArgumentType {
            function: f.name.to_string(),
            argument: name.to_string(),
            expected: "string".to_string(),
            got: "token".to_string(),
        }),
    }
}

fn get_token_argument(f: &function_string::Function, name: &str) -> Result<String> {
    let arg = f
        .arguments
        .get(name)
        .ok_or_else(|| Error::MissingArgument {
            function: f.name.clone(),
            argument: name.to_string(),
        })?;

    match arg {
        function_string::ArgumentValue::Token(t) => Ok(t.clone()),
        function_string::ArgumentValue::String(_) => Err(Error::IncorrectArgumentType {
            function: f.name.to_string(),
            argument: name.to_string(),
            expected: "token".to_string(),
            got: "string".to_string(),
        }),
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    mod parse {
        #[cfg(test)]
        use super::*;

        mod script {
            #[cfg(test)]
            use super::*;

            #[test]
            fn succeeds_when_function_is_script() {
                let result = parse("shell,script(name=\"example-script\")");
                assert_eq!(
                    result,
                    Ok(CodeBlockType::Script(ScriptName(
                        "example-script".to_string()
                    )))
                )
            }

            #[test]
            fn fails_when_name_is_missing() {
                let result = parse("shell,script()");
                assert_eq!(
                    result,
                    Err(Error::MissingArgument {
                        function: "script".to_string(),
                        argument: "name".to_string()
                    })
                )
            }
        }

        mod verify {
            #[cfg(test)]
            use super::*;

            #[test]
            fn succeeds_when_function_is_verify_and_stream_is_output() {
                let result = parse(",verify(script_name=\"example-script\", stream=output)");
                assert_eq!(
                    result,
                    Ok(CodeBlockType::Verify(Source {
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
                    Ok(CodeBlockType::Verify(Source {
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
                    Ok(CodeBlockType::Verify(Source {
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
                        function: "verify".to_string(),
                        argument: "stream".to_string(),
                        expected: "output, stdout or stderr".to_string(),
                        got: "unknown".to_string(),
                    })
                )
            }

            #[test]
            fn fails_when_script_name_is_missing() {
                let result = parse("shell,verify(stream=stderr)");
                assert_eq!(
                    result,
                    Err(Error::MissingArgument {
                        function: "verify".to_string(),
                        argument: "script_name".to_string()
                    })
                )
            }

            #[test]
            fn fails_when_stream_is_missing() {
                let result = parse("shell,verify(script_name=\"the-script\")");
                assert_eq!(
                    result,
                    Err(Error::MissingArgument {
                        function: "verify".to_string(),
                        argument: "stream".to_string()
                    })
                )
            }
        }
    }
}
