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
