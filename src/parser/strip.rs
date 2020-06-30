extern crate comrak;
use self::comrak::nodes::NodeCodeBlock;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{format_commonmark, parse_document, Arena, ComrakOptions};

use super::code_block_info;
use super::error::Result;

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
