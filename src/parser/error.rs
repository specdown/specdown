use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    RootMustBeDocument,
    StringEncodingFailed(String),
    ParserFailed(String),
    UnknownFunction(String),
    MissingArgument {
        function: String,
        argument: String,
    },
    IncorrectArgumentType {
        function: String,
        argument: String,
        expected: String,
        got: String,
    },
    InvalidArgumentValue {
        function: String,
        argument: String,
        expected: String,
        got: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RootMustBeDocument => {
                write!(f, "RootMustBeDocument :: This error should never occur")
            }
            Self::StringEncodingFailed(msg) => {
                write!(f, "Failed to encode string. Got error: {}", msg)
            }
            Self::ParserFailed(msg) => write!(f, "The parser failed: {}", msg),
            Self::UnknownFunction(name) => write!(f, "Unknown function: {}", name),
            Self::MissingArgument { function, argument } => {
                write!(f, "Function {} requires argument {}", function, argument)
            }
            Self::IncorrectArgumentType {
                function,
                argument,
                expected,
                got,
            } => write!(
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
    use super::Error;

    #[test]
    fn display_root_must_be_document() {
        assert_eq!(
            format!("{}", Error::RootMustBeDocument),
            "RootMustBeDocument :: This error should never occur"
        );
    }

    #[test]
    fn display_string_encoding_failed() {
        assert_eq!(
            format!("{}", Error::StringEncodingFailed("message".to_string())),
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
                Error::MissingArgument {
                    function: "funcy".to_string(),
                    argument: "argy".to_string()
                }
            ),
            "Function funcy requires argument argy"
        );
    }

    #[test]
    fn display_incorrect_argument_type() {
        assert_eq!(
            format!(
                "{}",
                Error::IncorrectArgumentType {
                    function: "test_func".to_string(),
                    argument: "test_arg".to_string(),
                    expected: "token".to_string(),
                    got: "string".to_string()
                }
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
