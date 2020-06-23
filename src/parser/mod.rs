use crate::types::Action;

mod actions;
mod argument_value;
mod code_block_info;
mod error;
mod function;
mod function_string;
mod markdown;

use error::Result;

pub fn parse(markdown: &str) -> Result<Vec<Action>> {
    let elements = markdown::parse(markdown)?;

    elements
        .iter()
        .map(to_action)
        .collect::<Result<Vec<Option<Action>>>>()
        .map(|actions| actions.into_iter().filter_map(|x| x).collect())
}

fn to_action(element: &markdown::Element) -> Result<Option<Action>> {
    match element {
        markdown::Element::FencedCodeBlock { info, literal } => {
            actions::create_action(&info, literal.clone())
        }
    }
}

mod tests {}
