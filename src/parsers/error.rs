use super::function_string_parser;
use super::markdown;

use nom::error::{ErrorKind, FromExternalError, ParseError};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("{0}")]
    FunctionStringParser(#[from] function_string_parser::Error),
    #[error("{0}")]
    MarkdownParser(#[from] markdown::Error),
    #[error("The parser failed: {0}")]
    ParserFailed(String),
    #[error("Unknown function: {0}")]
    UnknownFunction(String),
    #[error("Argument {argument} for function {function} must be {expected}, got {got}")]
    InvalidArgumentValue {
        function: String,
        argument: String,
        expected: String,
        got: String,
    },
}

impl ParseError<&str> for Error {
    fn from_error_kind(input: &str, kind: ErrorKind) -> Self {
        Self::ParserFailed(format!(
            "Failed parsing function from '{input}' :: {kind:?}"
        ))
    }

    fn append(input: &str, kind: ErrorKind, other: Self) -> Self {
        Self::ParserFailed(format!(
            "Failed parsing function from '{input}' :: {kind:?} (follows: {other}"
        ))
    }
}

impl FromExternalError<&str, Self> for Error {
    fn from_external_error(_input: &str, _kind: ErrorKind, e: Self) -> Self {
        e
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
