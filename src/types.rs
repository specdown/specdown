#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Stream {
    StdOut,
    StdErr,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetOs(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Source {
    pub name: Option<ScriptName>,
    pub stream: Stream,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptCode(pub String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyValue(pub String);

impl From<VerifyValue> for String {
    fn from(verify_value: VerifyValue) -> Self {
        let VerifyValue(value) = verify_value;
        value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FilePath(pub String);

impl From<FilePath> for String {
    fn from(file_path: FilePath) -> Self {
        let FilePath(value) = file_path;
        value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileContent(pub String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OutputExpectation {
    Any,
    StdOut,
    StdErr,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScriptAction {
    pub script_name: Option<ScriptName>,
    pub script_code: ScriptCode,
    pub expected_exit_code: Option<ExitCode>,
    pub expected_output: OutputExpectation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateFileAction {
    pub file_path: FilePath,
    pub file_content: FileContent,
}

/// A readiness condition for a `background` block's `ready_when` argument.
///
/// When set, the runner spawns the background script (non-blocking) and then
/// polls this condition until it is satisfied before proceeding to the next
/// action. This replaces the fragile `sleep 1` pattern where a test author
/// has to guess how long a server takes to bind a port or write a readiness
/// file.
///
/// The condition string is parsed from the `ready_when` argument. Supported
/// forms:
/// - `file:<path>` - succeeds when the given path exists on disk.
/// - `port:<n>` - succeeds when a TCP connection to `127.0.0.1:<n>` succeeds.
/// - `exit:<shell command>` - succeeds when the shell command exits with
///   code 0.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReadyWhen {
    /// Succeeds when the file at the given path exists.
    FileExists(FilePath),
    /// Succeeds when a TCP connection to `127.0.0.1:<port>` can be opened.
    PortOpen(u16),
    /// Succeeds when the given shell command exits with code 0.
    CheckExitZero(ScriptCode),
}

/// How many seconds to wait for a `ready_when` condition before failing.
///
/// This is the default timeout used when a `ready_when` condition is set but
/// no explicit `timeout_secs` argument is provided.
pub const DEFAULT_READY_WHEN_TIMEOUT_SECS: u32 = 30;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackgroundAction {
    pub script_name: Option<ScriptName>,
    pub script_code: ScriptCode,
    /// When set, the runner polls this condition after spawning and blocks
    /// until it is satisfied (or `timeout_secs` elapses). When `None`, the
    /// block behaves exactly as before - spawn and proceed immediately.
    pub ready_when: Option<ReadyWhen>,
    /// Readiness timeout in seconds. Defaults to
    /// [`DEFAULT_READY_WHEN_TIMEOUT_SECS`] when `ready_when` is set and this
    /// is `None`.
    pub timeout_secs: Option<u32>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Script(ScriptAction),
    Verify(VerifyAction),
    CreateFile(CreateFileAction),
    Background(BackgroundAction),
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
                expected_value: VerifyValue(String::new()),
            };

            assert_eq!(
                VerifyAction {
                    source: Source {
                        name: Some(ScriptName("new_name".to_string())),
                        stream: Stream::StdOut,
                    },
                    expected_value: VerifyValue(String::new())
                },
                action.with_script_name(Some(ScriptName("new_name".to_string())))
            );
        }
    }
}
