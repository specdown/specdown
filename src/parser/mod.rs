use std::string::{String, FromUtf8Error};

use comrak::{parse_document, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue, NodeCodeBlock};

use crate::types::Action;

mod actions;

#[derive(Debug, PartialEq)]
pub enum Error {
    RootMustBeDocument,
    StringEncodingFailed(FromUtf8Error),
}

type Result<T> = std::result::Result<T, Error>;

pub fn parse(markdown: String) -> Result<Vec<Action>> {
    let arena = Arena::new();
    let root = parse_document(&arena, &markdown, &ComrakOptions::default());
    let actions = extract_actions(root)?;

    print!("Found {} actions", actions.len());

    Ok(actions)
}

fn extract_actions<'a>(root: &'a AstNode<'a>) -> Result<Vec<Action>> {
    let node_value = &root.data.borrow_mut().value;

    match node_value {
        NodeValue::Document => Ok(root.children()),
        _ => Err(Error::RootMustBeDocument),
    }?
    .filter_map(to_codeblock)
    .map(to_action)
    .collect()
}

fn to_codeblock<'a>(node: &'a AstNode<'a>) -> Option<NodeCodeBlock> {
    match node.data.borrow().value.clone() {
        NodeValue::CodeBlock(block) => Some(block),
        _ => None,
    }
}

fn to_action(block: NodeCodeBlock) -> Result<Action> {
    let (info, literal) = node_block_to_components(block)?;
    actions::create_action(info, literal)
}

fn node_block_to_components(block: NodeCodeBlock) -> Result<(String, String)> {
    let info = char_vec_to_string(&block.info)?;
    let literal = char_vec_to_string(&block.literal)?;

    Ok((info, literal))
}

fn char_vec_to_string(chars: &[u8]) -> Result<String> {
    match String::from_utf8(chars.to_vec()) {
        Ok(string) => Ok(string),
        Err(err) => Err(Error::StringEncodingFailed(err))
    }
}

mod tests {

}