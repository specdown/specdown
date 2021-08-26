#[derive(Debug, PartialEq)]
pub enum Stream {
    StdOut,
    StdErr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptName(pub String);

#[derive(Debug, PartialEq)]
pub struct Source {
    pub name: ScriptName,
    pub stream: Stream,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ScriptCode(pub String);

#[derive(Debug, PartialEq)]
pub struct VerifyValue(pub String);

#[derive(Debug, PartialEq)]
pub struct FilePath(pub String);

#[derive(Debug, PartialEq)]
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
pub struct ScriptAction {
    pub script_name: ScriptName,
    pub script_code: ScriptCode,
    pub expected_exit_code: Option<ExitCode>,
}

#[derive(Debug, PartialEq)]
pub struct VerifyAction {
    pub source: Source,
    pub expected_value: VerifyValue,
}

#[derive(Debug, PartialEq)]
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
    use super::ExitCode;

    mod exit_code {
        use super::ExitCode;

        #[test]
        fn converts_from_exit_code_into_i32() {
            let value: i32 = ExitCode(10).into();
            assert_eq!(value, 10);
        }

        #[test]
        fn converts_from_exit_code_into_string() {
            let value: String = ExitCode(10).into();
            assert_eq!(value, String::from("10"));
        }
    }
}
