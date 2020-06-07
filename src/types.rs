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

#[derive(Debug, PartialEq)]
pub enum Action {
    Script(ScriptName, ScriptCode),
    Verify(Source, VerifyValue),
    CreateFile(FilePath, FileContent),
}
