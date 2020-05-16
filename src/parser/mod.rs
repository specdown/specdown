use comrak::nodes::{AstNode, NodeCodeBlock, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};

use crate::types::Action;

mod actions;
mod code_block_info;
mod error;
mod function_string;

use error::{Error, Result};

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

mod tests {}
