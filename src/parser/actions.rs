use crate::parser::blockquote_info;
use crate::parser::{Error, Result};
use crate::types::{Action, ScriptCode, VerifyValue};

pub fn create_action(info: &str, literal: String) -> Result<Action> {
    let block = blockquote_info::parse(&info).map_err(Error::BlockQuoteParsingFailed)?;

    match block {
        blockquote_info::BlockQuoteTypes::Script(name) => {
            Ok(Action::Script(name, ScriptCode(literal)))
        }
        blockquote_info::BlockQuoteTypes::Verify(source) => {
            Ok(Action::Verify(source, VerifyValue(literal)))
        }
    }
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[cfg(test)]
    use crate::types::{ScriptName, Source, Stream};

    #[test]
    fn create_action_for_script() {
        assert_eq!(
            create_action(
                "shell,script(name=\"script-name\")",
                "code".to_string(),
            ),
            Ok(Action::Script(
                ScriptName("script-name".to_string()),
                ScriptCode("code".to_string())
            )) as Result<Action>
        );
    }

    // #[test]
    // fn create_action_for_script_with_spaces_in_info() {
    //     assert_eq!(
    //         create_action(
    //             "zsh , script ( name = \"spaced-script\" )".to_string(),
    //             "spaced code".to_string()
    //         ),
    //         Ok(
    //             Action::Script(
    //                 ScriptName("spaced-script".to_string()),
    //                 ScriptCode("spaced code".to_string())
    //             )
    //         ) as Result<Action>
    //     );
    // }

    #[test]
    fn create_action_for_verify() {
        assert_eq!(
            create_action(
                ",verify(script_name=\"script-name\", stream=output)",
                "value".to_string()
            ),
            Ok(Action::Verify(
                Source {
                    name: ScriptName("script-name".to_string()),
                    stream: Stream::Output,
                },
                VerifyValue("value".to_string())
            )) as Result<Action>
        );
    }
}
