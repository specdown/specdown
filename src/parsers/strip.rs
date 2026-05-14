use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_commonmark, parse_document, Arena, Options};

use super::code_block_info;

pub fn strip(markdown: &str) -> String {
    let arena = Arena::new();

    let root = parse_document(&arena, markdown, &Options::default());

    iter_nodes(root, &|node| {
        if let NodeValue::CodeBlock(ref mut block) = node.data.borrow_mut().value {
            let info_string = block.info.clone();
            let language = code_block_info::parse(&info_string)
                .expect("To parse codeblock info")
                .language;
            block.info = language;
        }
    });

    let mut result = String::new();
    format_commonmark(root, &Options::default(), &mut result).unwrap();
    result
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

            assert_eq!(strip(markdown), expected.to_string());
        }
    }
}
