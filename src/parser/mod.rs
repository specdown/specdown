use crate::types::Action;

mod actions;
mod argument_value;
mod code_block_info;
mod error;
mod function;
mod function_string;
mod markdown;

use error::Result;

extern crate comrak;
use self::comrak::nodes::NodeCodeBlock;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_commonmark, parse_document, Arena, ComrakOptions};

pub fn parse(markdown: &str) -> Result<Vec<Action>> {
    let elements = markdown::parse(markdown)?;

    elements
        .iter()
        .map(to_action)
        .collect::<Result<Vec<Option<Action>>>>()
        .map(|actions| actions.into_iter().filter_map(|x| x).collect())
}

pub fn strip(markdown: &str) -> Result<String> {
    let arena = Arena::new();

    let root = parse_document(&arena, markdown, &ComrakOptions::default());

    iter_nodes(root, &|node| {
        if let NodeValue::CodeBlock(NodeCodeBlock { ref mut info, .. }) =
            &mut node.data.borrow_mut().value
        {
            let info_string = String::from_utf8((*info).to_vec()).expect("UTF8 string");
            let (language, _function) =
                code_block_info::parse(&info_string).expect("To parse codeblock info");
            *info = Vec::from(language)
        }
    });

    let mut result = vec![];
    format_commonmark(root, &ComrakOptions::default(), &mut result).unwrap();
    Ok(String::from_utf8(result).unwrap())
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
where
    F: Fn(&'a AstNode<'a>),
{
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

fn to_action(element: &markdown::Element) -> Result<Option<Action>> {
    match element {
        markdown::Element::FencedCodeBlock { info, literal } => {
            actions::create_action(&info, literal.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::strip;

    mod strip {
        use super::strip;
        use indoc::indoc;

        #[test]
        fn test_strip() {
            let markdown = indoc!(
                "
                # Header

                ```shell, script(name=\"something\")
                run
                ```
                "
            );

            let expected = indoc!(
                "
                # Header

                ``` shell
                run
                ```
                "
            );

            assert_eq!(strip(markdown), Ok(expected.to_string()))
        }
    }
}
