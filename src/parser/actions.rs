use super::code_block_info;
use super::error::Result;
use crate::types::{Action, FileContent, ScriptCode, VerifyValue};

pub fn create_action(info: &str, literal: String) -> Result<Option<Action>> {
    let (_, block) = code_block_info::parse(&info)?;

    Ok(match block {
        code_block_info::CodeBlockType::Script(script_name, expected_exit_code) => {
            Some(Action::Script {
                script_name,
                script_code: ScriptCode(literal),
                expected_exit_code,
            })
        }
        code_block_info::CodeBlockType::Verify(source) => Some(Action::Verify {
            source,
            expected_value: VerifyValue(literal),
        }),
        code_block_info::CodeBlockType::CreateFile(file_path) => Some(Action::CreateFile {
            file_path,
            file_content: FileContent(literal),
        }),
        code_block_info::CodeBlockType::Skip() => None,
    })
}

#[cfg(test)]
mod tests {
    use super::{create_action, Action, FileContent, ScriptCode, VerifyValue};
    use crate::types::{FilePath, ScriptName, Source, Stream};

    #[test]
    fn create_action_for_script() {
        assert_eq!(
            create_action("shell,script(name=\"script-name\")", "code".to_string()),
            Ok(Some(Action::Script {
                script_name: ScriptName("script-name".to_string()),
                script_code: ScriptCode("code".to_string()),
                expected_exit_code: None
            }))
        )
    }

    #[test]
    fn create_action_for_verify() {
        assert_eq!(
            create_action(
                ",verify(script_name=\"script-name\", stream=stdout)",
                "value".to_string()
            ),
            Ok(Some(Action::Verify {
                source: Source {
                    name: ScriptName("script-name".to_string()),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("value".to_string())
            }))
        )
    }

    #[test]
    fn create_action_for_file() {
        assert_eq!(
            create_action(",file(path=\"file.txt\")", "content".to_string()),
            Ok(Some(Action::CreateFile {
                file_path: FilePath("file.txt".to_string()),
                file_content: FileContent("content".to_string())
            }))
        )
    }

    #[test]
    fn create_action_for_skip() {
        assert_eq!(create_action(",skip()", "content".to_string()), Ok(None))
    }
}
