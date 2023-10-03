use crate::types::Action;

mod actions;
mod code_block_info;
mod code_block_type;
mod error;
mod function_string_parser;
mod markdown;
mod strip;

use error::Result;

pub use strip::strip;

pub use error::Error;

pub fn parse(markdown: &str) -> Result<Vec<Action>> {
    markdown::parse(markdown)?
        .iter()
        .map(to_action)
        .collect::<Result<Vec<Option<Action>>>>()
        .map(|actions| actions.into_iter().flatten().collect())
}

fn to_action(element: &markdown::Element) -> Result<Option<Action>> {
    match element {
        markdown::Element::FencedCodeBlock { info, literal } => {
            let code_block_type = code_block_info::parse(info)?.extra;
            Ok(actions::create_action(&code_block_type, literal.clone()))
        }
    }
}
