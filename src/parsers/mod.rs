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
            if !info.contains(',') {
                return Ok(None);
            }
            let code_block_type = code_block_info::parse(info)?.extra;
            Ok(actions::create_action(&code_block_type, literal.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::parse;

    #[test]
    fn a_code_block_whose_info_string_is_not_a_specdown_function_is_ignored() {
        let markdown = indoc! {r"
            # Title

            ```specdown
            this is just a fenced block named specdown
            ```
        "};

        assert_eq!(parse(markdown), Ok(vec![]));
    }

    #[test]
    fn a_code_block_with_a_language_but_no_specdown_function_is_ignored() {
        let markdown = indoc! {r"
            ```rust
            fn main() {}
            ```
        "};

        assert_eq!(parse(markdown), Ok(vec![]));
    }
}
