use super::code_block_info;
use super::error::Result;
use crate::parser::code_block_info::{CodeBlockInfo, CodeBlockType, ScriptCodeBlock};
use crate::types::{
    Action, CreateFileAction, FileContent, ScriptAction, ScriptCode, Source, TargetOs,
    VerifyAction, VerifyValue,
};
use std::env::consts::OS;

pub fn create_action(info: &str, literal: String) -> Result<Option<Action>> {
    let CodeBlockInfo {
        code_block_type, ..
    } = code_block_info::parse(info)?;

    Ok(from_code_block_type(literal, &code_block_type))
}

fn from_code_block_type(literal: String, code_block_type: &CodeBlockType) -> Option<Action> {
    match code_block_type {
        code_block_info::CodeBlockType::Script(code_block) => {
            Some(Action::Script(to_script_action(code_block, literal)))
        }
        code_block_info::CodeBlockType::Verify(source) => {
            to_verify_action(source, literal).map(Action::Verify)
        }
        code_block_info::CodeBlockType::CreateFile(ref file_path) => {
            Some(Action::CreateFile(CreateFileAction {
                file_path: file_path.clone(),
                file_content: FileContent(literal),
            }))
        }
        code_block_info::CodeBlockType::Skip() => None,
    }
}

fn to_script_action(code_block: &ScriptCodeBlock, literal: String) -> ScriptAction {
    let ScriptCodeBlock {
        script_name,
        expected_exit_code,
        expected_output,
    } = code_block;

    ScriptAction {
        script_name: script_name.clone(),
        script_code: ScriptCode(literal),
        expected_exit_code: *expected_exit_code,
        expected_output: expected_output.clone(),
    }
}

fn to_verify_action(source: &Source, literal: String) -> Option<VerifyAction> {
    match source {
        Source {
            target_os: None, ..
        } => Some(VerifyAction {
            source: source.clone(),
            expected_value: VerifyValue(literal),
        }),
        Source {
            target_os: Some(TargetOs(ref target_os)),
            ..
        } if target_os_matches_current(target_os) => Some(VerifyAction {
            source: source.clone(),
            expected_value: VerifyValue(literal),
        }),
        Source {
            target_os: Some(_), ..
        } => None,
    }
}

fn target_os_matches_current(target_os: &str) -> bool {
    if target_os == OS {
        return true;
    }

    target_os.starts_with('!') && target_os != format!("!{}", OS)
}

#[cfg(test)]
mod tests {
    use super::{create_action, Action, FileContent, ScriptCode, VerifyValue};
    use crate::types::{
        CreateFileAction, FilePath, OutputExpectation, ScriptAction, ScriptName, Source, Stream,
        TargetOs, VerifyAction,
    };

    #[test]
    fn create_action_for_script() {
        assert_eq!(
            create_action("shell,script(name=\"script-name\")", "code".to_string()),
            Ok(Some(Action::Script(ScriptAction {
                script_name: Some(ScriptName("script-name".to_string())),
                script_code: ScriptCode("code".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            })))
        );
    }

    #[test]
    fn create_action_for_verify() {
        assert_eq!(
            create_action(
                ",verify(script_name=\"script-name\", stream=stdout)",
                "value".to_string(),
            ),
            Ok(Some(Action::Verify(VerifyAction {
                source: Source {
                    name: Some(ScriptName("script-name".to_string())),
                    stream: Stream::StdOut,
                    target_os: None,
                },
                expected_value: VerifyValue("value".to_string()),
            })))
        );
    }

    #[test]
    fn create_action_for_verify_that_is_skipped() {
        assert_eq!(
            create_action(
                ",verify(script_name=\"script-name\", target_os=\"fake-os\")",
                "value".to_string(),
            ),
            Ok(None)
        );
    }

    #[test]
    fn create_action_for_verify_that_is_negated() {
        assert_eq!(
            create_action(
                ",verify(script_name=\"script-name\", target_os=\"!fake-os\")",
                "value".to_string(),
            ),
            Ok(Some(Action::Verify(VerifyAction {
                source: Source {
                    name: Some(ScriptName("script-name".to_string())),
                    stream: Stream::StdOut,
                    target_os: Some(TargetOs("!fake-os".to_string())),
                },
                expected_value: VerifyValue("value".to_string()),
            })))
        );
    }

    #[test]
    fn create_action_for_file() {
        assert_eq!(
            create_action(",file(path=\"file.txt\")", "content".to_string()),
            Ok(Some(Action::CreateFile(CreateFileAction {
                file_path: FilePath("file.txt".to_string()),
                file_content: FileContent("content".to_string()),
            })))
        );
    }

    #[test]
    fn create_action_for_skip() {
        assert_eq!(create_action(",skip()", "content".to_string()), Ok(None));
    }
}
