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
