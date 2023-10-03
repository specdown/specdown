use comrak::arena_tree::Children;
use comrak::nodes::{Ast, AstNode, NodeCodeBlock, NodeValue};
use comrak::{parse_document, Arena, ComrakOptions};
use std::cell::RefCell;

#[derive(Clone, Debug, Eq, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("RootMustBeDocument :: This error should never occur")]
    RootMustBeDocument,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Element {
    FencedCodeBlock { info: String, literal: String },
}

pub fn parse(markdown: &str) -> Result<Vec<Element>, Error> {
    let arena = Arena::new();
    let root = parse_document(&arena, markdown, &ComrakOptions::default());
    extract_elements(root)
}

fn extract_elements<'a>(root: &'a AstNode<'a>) -> Result<Vec<Element>, Error> {
    Ok(get_root_children(root)?.filter_map(to_element).collect())
}

fn get_root_children<'a>(root: &'a AstNode<'a>) -> Result<Children<'_, RefCell<Ast>>, Error> {
    match &root.data.borrow_mut().value {
        NodeValue::Document => Ok(root.children()),
        _ => Err(Error::RootMustBeDocument),
    }
}

fn to_element<'a>(node: &'a AstNode<'a>) -> Option<Element> {
    match node.data.borrow().value.clone() {
        NodeValue::CodeBlock(block) => Some(block)
            .filter(|b| b.fenced)
            .map(|b| to_fenced_code_block_element(&b)),
        _ => None,
    }
}

fn to_fenced_code_block_element(block: &NodeCodeBlock) -> Element {
    let (info, literal) = node_block_to_components(block);
    Element::FencedCodeBlock { info, literal }
}

fn node_block_to_components(block: &NodeCodeBlock) -> (String, String) {
    (block.info.clone(), block.literal.clone())
}

#[cfg(test)]
mod tests {
    use super::{parse, Element};
    use indoc::indoc;

    #[test]
    fn no_actions_returned_when_not_code_blocks_in_markdown() {
        let markdown = indoc!("# This is markdown");

        assert_eq!(parse(markdown), Ok(vec![]));
    }

    #[test]
    fn fenced_block_quotes_are_returned_when_the_exist_in_the_markdown() {
        let markdown = indoc!(
            "# This is markdown

            ```info1
            literal1
            ```

            content

            ```info2
            literal2
            ```

            footer
            "
        );

        assert_eq!(
            parse(markdown),
            Ok(vec![
                Element::FencedCodeBlock {
                    info: "info1".to_string(),
                    literal: "literal1\n".to_string(),
                },
                Element::FencedCodeBlock {
                    info: "info2".to_string(),
                    literal: "literal2\n".to_string(),
                },
            ])
        );
    }

    #[test]
    fn it_does_not_return_an_element_when_a_code_bloc_is_not_fenced() {
        let markdown = "# Non-fenced\n    this code block is not fenced";

        assert_eq!(parse(markdown), Ok(vec![]));
    }
}
