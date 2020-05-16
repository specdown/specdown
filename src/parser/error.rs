use std::fmt;

use crate::parser::code_block_info;

#[derive(Debug, PartialEq)]
pub enum Error {
    RootMustBeDocument,
    StringEncodingFailed(String),
    CodeBlockParsingFailed(code_block_info::Error),
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
            Self::CodeBlockParsingFailed(error) => {
                write!(f, "Failed to parse code block: {}", error)
            }
        }
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn display_root_must_be_document() {
        assert_eq!(
            format!("{}", Error::RootMustBeDocument),
            "RootMustBeDocument :: This error should never occur"
        )
    }

    #[test]
    fn display_string_encoding_failed() {
        assert_eq!(
            format!("{}", Error::StringEncodingFailed("message".to_string())),
            "Failed to encode string. Got error: message"
        )
    }

    #[test]
    fn display_code_block_parsing_failed() {
        assert_eq!(
            format!(
                "{}",
                Error::CodeBlockParsingFailed(code_block_info::Error::UnknownFunction(
                    "xzy".to_string()
                ))
            ),
            "Failed to parse code block: Unknown function: xzy"
        )
    }
}
