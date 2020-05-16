use std::fmt;

use comrak::nodes::{AstNode, NodeCodeBlock, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};

use crate::types::Action;

mod actions;
mod blockquote_info;
mod function_string;

#[derive(Debug, PartialEq)]
pub enum Error {
    RootMustBeDocument,
    StringEncodingFailed(String),
    BlockQuoteParsingFailed(blockquote_info::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RootMustBeDocument => {
                write!(f, "RootMustBeDocument :: This error should never occur")
            }
            Error::StringEncodingFailed(msg) => {
                write!(f, "Failed to encode string. Got error: {}", msg)
            }
            Error::BlockQuoteParsingFailed(error) => {
                write!(f, "Failed to parse blockquote: {}", error)
            }
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn parse(markdown: &str) -> Result<Vec<Action>> {
    let arena = Arena::new();
    let root = parse_document(&arena, &markdown, &ComrakOptions::default());
    let actions = extract_actions(root)?;

    Ok(actions)
}

fn extract_actions<'a>(root: &'a AstNode<'a>) -> Result<Vec<Action>> {
    let node_value = &root.data.borrow_mut().value;

    match node_value {
        NodeValue::Document => Ok(root.children()),
        _ => Err(Error::RootMustBeDocument),
    }?
    .filter_map(|node| to_codeblock(node).map(|block| to_action(&block)))
    .collect()
}

fn to_codeblock<'a>(node: &'a AstNode<'a>) -> Option<NodeCodeBlock> {
    match node.data.borrow().value.clone() {
        NodeValue::CodeBlock(block) => Some(block),
        _ => None,
    }
}

fn to_action(block: &NodeCodeBlock) -> Result<Action> {
    let (info, literal) = node_block_to_components(&block)?;
    actions::create_action(&info, literal)
}

fn node_block_to_components(block: &NodeCodeBlock) -> Result<(String, String)> {
    let info = char_vec_to_string(&block.info)?;
    let literal = char_vec_to_string(&block.literal)?;

    Ok((info, literal))
}

fn char_vec_to_string(chars: &[u8]) -> Result<String> {
    match String::from_utf8(chars.to_vec()) {
        Ok(string) => Ok(string),
        Err(err) => Err(Error::StringEncodingFailed(err.to_string())),
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    mod errors {
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
        fn display_blockquote_parsing_failed() {
            assert_eq!(
                format!(
                    "{}",
                    Error::BlockQuoteParsingFailed(blockquote_info::Error::UnknownFunction(
                        "xzy".to_string()
                    ))
                ),
                "Failed to parse blockquote: Unknown function: xzy"
            )
        }
    }
}
