#[derive(Debug, PartialEq)]
pub enum Stream {
    StdOut,
    StdErr,
}

#[derive(Debug, PartialEq)]
pub struct ScriptName(pub String);

#[derive(Debug, PartialEq)]
pub struct Source {
    pub name: ScriptName,
    pub stream: Stream,
}

#[derive(Debug, PartialEq)]
pub struct ScriptCode(pub String);

#[derive(Debug, PartialEq)]
pub struct VerifyValue(pub String);

#[derive(Debug, PartialEq)]
pub struct FilePath(pub String);

#[derive(Debug, PartialEq)]
pub struct FileContent(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct ExitCode(pub i32);

#[derive(Debug, PartialEq)]
pub enum Action {
    Script {
        script_name: ScriptName,
        script_code: ScriptCode,
        expected_exit_code: Option<ExitCode>,
    },
    Verify {
        source: Source,
        expected_value: VerifyValue,
    },
    CreateFile {
        file_path: FilePath,
        file_content: FileContent,
    },
}
