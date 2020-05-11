use crate::types::{Action, ScriptCode, ScriptName, Source, Stream, VerifyValue};
use crate::parser::Result;

pub fn create_action(info: String, literal: String) -> Result<Action> {
    if !info.contains("verify") {
        Ok(Action::Script(
            ScriptName("script-name".to_string()),
            ScriptCode("code".to_string())
        ))
    } else {
        Ok(Action::Verify(
            Source {
                name: ScriptName("script-name".to_string()),
                stream: Stream::Output
            },
            VerifyValue("value".to_string())
        ))
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_action_for_script() {
        assert_eq!(
            create_action(
                "shell,script(name=\"script-name\")".to_string(),
                "code".to_string()
            ),
            Ok(
                Action::Script(
                    ScriptName("script-name".to_string()),
                    ScriptCode("code".to_string())
                )
            ) as Result<Action>
        );
    }

    #[test]
    fn create_action_for_verify() {
        assert_eq!(
            create_action(
                ",verify(source_name=\"script-name\", stream=output)".to_string(),
                "value".to_string()
            ),
            Ok(
                Action::Verify(
                    Source {
                        name: ScriptName("script-name".to_string()),
                        stream: Stream::Output,
                    },
                    VerifyValue("value".to_string())
                )
            ) as Result<Action>
        );
    }
}