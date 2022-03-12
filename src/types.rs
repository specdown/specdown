#[derive(Clone, Debug, PartialEq)]
pub enum Stream {
    StdOut,
    StdErr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TargetOs(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptName(pub String);

impl From<ScriptName> for String {
    fn from(script_name: ScriptName) -> Self {
        let ScriptName(value) = script_name;
        value
    }
}

impl From<&ScriptName> for String {
    fn from(script_name: &ScriptName) -> Self {
        let ScriptName(value) = script_name;
        value.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Source {
    pub name: Option<ScriptName>,
    pub stream: Stream,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptCode(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct VerifyValue(pub String);

impl From<VerifyValue> for String {
    fn from(verify_value: VerifyValue) -> Self {
        let VerifyValue(value) = verify_value;
        value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilePath(pub String);

impl From<FilePath> for String {
    fn from(file_path: FilePath) -> Self {
        let FilePath(value) = file_path;
        value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileContent(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExitCode(pub i32);

impl From<ExitCode> for String {
    fn from(exit_code: ExitCode) -> Self {
        let ExitCode(value) = exit_code;
        value.to_string()
    }
}

impl From<ExitCode> for i32 {
    fn from(exit_code: ExitCode) -> Self {
        let ExitCode(value) = exit_code;
        value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum OutputExpectation {
    Any,
    StdOut,
    StdErr,
    None,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptAction {
    pub script_name: Option<ScriptName>,
    pub script_code: ScriptCode,
    pub expected_exit_code: Option<ExitCode>,
    pub expected_output: OutputExpectation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VerifyAction {
    pub source: Source,
    pub expected_value: VerifyValue,
}

impl VerifyAction {
    pub fn with_script_name(&self, script_name: Option<ScriptName>) -> Self {
        Self {
            source: Source {
                name: script_name,
                stream: self.source.stream.clone(),
            },
            expected_value: self.expected_value.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateFileAction {
    pub file_path: FilePath,
    pub file_content: FileContent,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Script(ScriptAction),
    Verify(VerifyAction),
    CreateFile(CreateFileAction),
}

#[cfg(test)]
mod tests {
    use super::{ExitCode, FilePath, ScriptName, Source, Stream, VerifyAction, VerifyValue};

    mod script_name {
        use super::ScriptName;

        #[test]
        fn converts_to_string_from_script_name() {
            assert_eq!(
                String::from(ScriptName("name".to_string())),
                String::from("name")
            );
        }

        #[test]
        fn converts_to_string_from_script_name_reference() {
            assert_eq!(
                String::from(&ScriptName("name".to_string())),
                String::from("name")
            );
        }
    }

    mod verify_value {
        use super::VerifyValue;

        #[test]
        fn converts_from_verify_value_into_string() {
            assert_eq!(
                String::from(VerifyValue("contents".to_string())),
                String::from("contents")
            );
        }
    }

    mod file_path {
        use super::FilePath;

        #[test]
        fn converts_from_file_path_into_string() {
            assert_eq!(
                String::from(FilePath("abc.txt".to_string())),
                String::from("abc.txt")
            );
        }
    }

    mod exit_code {
        use super::ExitCode;

        #[test]
        fn converts_from_exit_code_into_i32() {
            assert_eq!(i32::from(ExitCode(10)), 10);
        }

        #[test]
        fn converts_from_exit_code_into_string() {
            assert_eq!(String::from(ExitCode(10)), String::from("10"));
        }
    }

    mod verify_action {
        use super::{Source, Stream, VerifyAction, VerifyValue};
        use crate::types::ScriptName;

        #[test]
        fn with_script_name_returns_an_instance_with_script_name_updated() {
            let action = VerifyAction {
                source: Source {
                    name: None,
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("".to_string()),
            };

            assert_eq!(
                VerifyAction {
                    source: Source {
                        name: Some(ScriptName("new_name".to_string())),
                        stream: Stream::StdOut,
                    },
                    expected_value: VerifyValue("".to_string())
                },
                action.with_script_name(Some(ScriptName("new_name".to_string())))
            );
        }
    }
}
