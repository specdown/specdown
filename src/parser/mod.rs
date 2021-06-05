use crate::types::Action;

mod actions;
mod argument_value;
mod code_block_info;
mod error;
mod function;
mod function_string;
mod markdown;
mod strip;

use error::Result;

pub use strip::strip;

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
            actions::create_action(info, literal.clone())
        }
    }
}
