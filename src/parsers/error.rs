use super::function_string_parser;
use super::markdown;

use nom::error::{ErrorKind, FromExternalError, ParseError};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    FunctionStringParser(function_string_parser::Error),
    MarkdownParser(markdown::Error),
    ParserFailed(String),
    UnknownFunction(String),
    InvalidArgumentValue {
        function: String,
        argument: String,
        expected: String,
        got: String,
    },
}

impl From<function_string_parser::Error> for Error {
    fn from(error: function_string_parser::Error) -> Self {
        Error::FunctionStringParser(error)
    }
}

impl From<markdown::Error> for Error {
    fn from(error: markdown::Error) -> Self {
        Error::MarkdownParser(error)
    }
}

impl ParseError<&str> for Error {
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        Error::ParserFailed(format!(
            "Failed parsing function from '{}' :: {:?}",
            input, kind
        ))
    }

    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
        Error::ParserFailed(format!(
            "Failed parsing function from '{}' :: {:?} (follows: {}",
            input, kind, other
        ))
    }
}

impl FromExternalError<&str, Error> for Error {
    fn from_external_error(_input: &str, _kind: ErrorKind, e: Error) -> Self {
        e
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MarkdownParser(markdown::Error::RootMustBeDocument) => {
                write!(f, "RootMustBeDocument :: This error should never occur")
            }
            Self::MarkdownParser(markdown::Error::StringEncodingFailed(msg)) => {
                write!(f, "Failed to encode string. Got error: {}", msg)
            }
            Self::ParserFailed(msg) => write!(f, "The parser failed: {}", msg),
            Self::UnknownFunction(name) => write!(f, "Unknown function: {}", name),
            Self::FunctionStringParser(function_string_parser::Error::MissingArgument {
                function,
                argument,
            }) => {
                write!(f, "Function {} requires argument {}", function, argument)
            }
            Self::FunctionStringParser(function_string_parser::Error::IncorrectArgumentType {
                function,
                argument,
                expected,
                got,
            }) => write!(
                f,
                "Function {} requires argument {} to be a {}, got {}",
                function, argument, expected, got
            ),
            Self::InvalidArgumentValue {
                function,
                argument,
                expected,
                got,
            } => write!(
                f,
                "Argument {} for function {} must be {}, got {}",
                argument, function, expected, got
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{function_string_parser, markdown, Error};

    #[test]
    fn display_root_must_be_document() {
        assert_eq!(
            format!(
                "{}",
                Error::MarkdownParser(markdown::Error::RootMustBeDocument)
            ),
            "RootMustBeDocument :: This error should never occur"
        );
    }

    #[test]
    fn display_string_encoding_failed() {
        assert_eq!(
            format!(
                "{}",
                Error::MarkdownParser(markdown::Error::StringEncodingFailed("message".to_string()))
            ),
            "Failed to encode string. Got error: message"
        );
    }

    #[test]
    fn display_parser_failed() {
        assert_eq!(
            format!("{}", Error::ParserFailed("reason".to_string())),
            "The parser failed: reason"
        );
    }

    #[test]
    fn display_unknown_function() {
        assert_eq!(
            format!("{}", Error::UnknownFunction("funcy".to_string())),
            "Unknown function: funcy"
        );
    }

    #[test]
    fn display_missing_argument() {
        assert_eq!(
            format!(
                "{}",
                Error::FunctionStringParser(function_string_parser::Error::MissingArgument {
                    function: "funcy".to_string(),
                    argument: "argy".to_string()
                })
            ),
            "Function funcy requires argument argy"
        );
    }

    #[test]
    fn display_incorrect_argument_type() {
        assert_eq!(
            format!(
                "{}",
                Error::FunctionStringParser(function_string_parser::Error::IncorrectArgumentType {
                    function: "test_func".to_string(),
                    argument: "test_arg".to_string(),
                    expected: "token".to_string(),
                    got: "string".to_string()
                })
            ),
            "Function test_func requires argument test_arg to be a token, got string"
        );
    }

    #[test]
    fn display_invalid_argument_value() {
        assert_eq!(
            format!(
                "{}",
                Error::InvalidArgumentValue {
                    function: "func".to_string(),
                    argument: "arg".to_string(),
                    expected: "true or false".to_string(),
                    got: "maybe".to_string()
                }
            ),
            "Argument arg for function func must be true or false, got maybe"
        );
    }
}
