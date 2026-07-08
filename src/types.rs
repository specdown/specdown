use std::convert::TryFrom;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackgroundAction {
    pub script_name: Option<ScriptName>,
    pub script_code: ScriptCode,
}

/// The name of a mock endpoint, used to pair request and response blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MockName(pub String);

impl From<MockName> for String {
    fn from(mock_name: MockName) -> Self {
        let MockName(value) = mock_name;
        value
    }
}

impl From<&MockName> for String {
    fn from(mock_name: &MockName) -> Self {
        let MockName(value) = mock_name;
        value.clone()
    }
}

impl std::fmt::Display for MockName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let MockName(value) = self;
        write!(f, "{value}")
    }
}

/// An HTTP status code. Validated to the range 100..=599 at parse time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StatusCode(pub u16);

/// Error returned when a status code is outside the valid range.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusCodeError {
    pub value: i32,
}

impl StatusCodeError {
    #[must_use]
    pub fn value(&self) -> i32 {
        self.value
    }
}

impl StatusCode {
    /// Parse an integer into a `StatusCode`, validating the HTTP range 100..=599.
    pub fn parse(value: i32) -> Result<Self, StatusCodeError> {
        if (100..=599).contains(&value) {
            Ok(StatusCode(u16::try_from(value).expect("validated range")))
        } else {
            Err(StatusCodeError { value })
        }
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode(200)
    }
}

/// A response header. The name is stored lower-cased per HTTP convention.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseHeader {
    pub name: String,
    pub value: String,
}

/// A delay in milliseconds applied before a mock response is sent.
/// Capped at `300_000` (5 minutes) at parse time.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DelayMillis(pub u32);

/// Error returned when a delay value exceeds the maximum.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DelayMillisError {
    pub value: i32,
}

impl DelayMillisError {
    #[must_use]
    pub fn value(&self) -> i32 {
        self.value
    }
}

/// Maximum allowed delay in milliseconds (5 minutes).
pub const MAX_DELAY_MILLIS: i32 = 300_000;

impl DelayMillis {
    /// Parse an integer into a `DelayMillis`, capping at `MAX_DELAY_MILLIS`.
    pub fn parse(value: i32) -> Result<Self, DelayMillisError> {
        if !(0..=MAX_DELAY_MILLIS).contains(&value) {
            return Err(DelayMillisError { value });
        }
        Ok(DelayMillis(
            u32::try_from(value).expect("validated non-negative"),
        ))
    }
}

impl std::fmt::Display for StatusCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid status code: {}", self.value())
    }
}

impl std::fmt::Display for DelayMillisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid delay: {}ms", self.value())
    }
}

/// The body of a mock response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseBody {
    /// Body taken from the code block literal.
    Literal(String),
    /// Reserved for a future `body="..."` argument — not wired in v1.
    Inline(String),
    /// No body (empty literal).
    Empty,
}

/// A parsed `response` code block — the raw values before action decoding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseCodeBlock {
    pub name: MockName,
    pub status: StatusCode,
    pub headers: Option<String>,
    pub content_type: Option<String>,
    pub delay: DelayMillis,
    pub body: ResponseBody,
}

/// A decoded response action — headers expanded, body resolved.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseAction {
    pub name: MockName,
    pub status: StatusCode,
    pub headers: Vec<ResponseHeader>,
    pub delay: DelayMillis,
    pub body: ResponseBody,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Script(ScriptAction),
    Verify(VerifyAction),
    CreateFile(CreateFileAction),
    Background(BackgroundAction),
    Response(ResponseAction),
}

#[cfg(test)]
mod tests {
    use super::{
        DelayMillis, ExitCode, FilePath, MockName, ResponseBody, ScriptName, Source, StatusCode,
        Stream, VerifyAction, VerifyValue,
    };

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

    mod mock_name {
        use super::MockName;

        #[test]
        fn converts_to_string_from_mock_name() {
            assert_eq!(
                String::from(MockName("my-mock".to_string())),
                String::from("my-mock")
            );
        }

        #[test]
        fn converts_to_string_from_mock_name_reference() {
            assert_eq!(
                String::from(&MockName("my-mock".to_string())),
                String::from("my-mock")
            );
        }

        #[test]
        fn displays_as_the_inner_string() {
            assert_eq!(format!("{}", MockName("hello".to_string())), "hello");
        }

        #[test]
        fn equal_when_inner_strings_are_equal() {
            assert_eq!(
                MockName("list-users".to_string()),
                MockName("list-users".to_string())
            );
        }

        #[test]
        fn not_equal_when_inner_strings_differ() {
            assert_ne!(
                MockName("list-users".to_string()),
                MockName("get-user".to_string())
            );
        }
    }

    mod status_code {
        use super::StatusCode;

        #[test]
        fn parse_accepts_a_valid_status_code() {
            assert_eq!(StatusCode(200), StatusCode::parse(200).unwrap());
        }

        #[test]
        fn parse_accepts_the_minimum_valid_status_code() {
            assert_eq!(StatusCode(100), StatusCode::parse(100).unwrap());
        }

        #[test]
        fn parse_accepts_the_maximum_valid_status_code() {
            assert_eq!(StatusCode(599), StatusCode::parse(599).unwrap());
        }

        #[test]
        fn parse_rejects_a_status_code_below_the_minimum() {
            assert!(StatusCode::parse(99).is_err());
        }

        #[test]
        fn parse_rejects_zero() {
            assert!(StatusCode::parse(0).is_err());
        }

        #[test]
        fn parse_rejects_a_negative_status_code() {
            assert!(StatusCode::parse(-1).is_err());
        }

        #[test]
        fn parse_rejects_a_status_code_above_the_maximum() {
            assert!(StatusCode::parse(600).is_err());
        }

        #[test]
        fn default_is_200() {
            assert_eq!(StatusCode(200), StatusCode::default());
        }

        #[test]
        fn equality_compares_the_inner_u16() {
            assert_eq!(StatusCode(404), StatusCode(404));
            assert_ne!(StatusCode(404), StatusCode(200));
        }

        #[test]
        fn parse_error_contains_the_rejected_value() {
            let err = StatusCode::parse(99).unwrap_err();
            assert_eq!(99, err.value());
        }
    }

    mod delay_millis {
        use super::DelayMillis;

        #[test]
        fn parse_accepts_zero() {
            assert_eq!(DelayMillis(0), DelayMillis::parse(0).unwrap());
        }

        #[test]
        fn parse_accepts_the_maximum() {
            assert_eq!(DelayMillis(300_000), DelayMillis::parse(300_000).unwrap());
        }

        #[test]
        fn parse_rejects_a_negative_value() {
            assert!(DelayMillis::parse(-1).is_err());
        }

        #[test]
        fn parse_rejects_a_value_above_the_maximum() {
            assert!(DelayMillis::parse(300_001).is_err());
        }

        #[test]
        fn default_is_zero() {
            assert_eq!(DelayMillis(0), DelayMillis::default());
        }

        #[test]
        fn equality_compares_the_inner_u32() {
            assert_eq!(DelayMillis(100), DelayMillis(100));
            assert_ne!(DelayMillis(100), DelayMillis(200));
        }

        #[test]
        fn parse_error_contains_the_rejected_value() {
            let err = DelayMillis::parse(-1).unwrap_err();
            assert_eq!(-1, err.value());
        }
    }

    mod response_body {
        use super::ResponseBody;

        #[test]
        fn literal_equality_compares_inner_string() {
            assert_eq!(
                ResponseBody::Literal("hello".to_string()),
                ResponseBody::Literal("hello".to_string())
            );
            assert_ne!(
                ResponseBody::Literal("hello".to_string()),
                ResponseBody::Literal("world".to_string())
            );
        }

        #[test]
        fn empty_is_equal_to_empty() {
            assert_eq!(ResponseBody::Empty, ResponseBody::Empty);
        }

        #[test]
        fn inline_is_not_equal_to_literal_with_same_content() {
            assert_ne!(
                ResponseBody::Inline("data".to_string()),
                ResponseBody::Literal("data".to_string())
            );
        }

        #[test]
        fn empty_is_not_equal_to_literal_with_empty_string() {
            assert_ne!(ResponseBody::Empty, ResponseBody::Literal(String::new()));
        }
    }
}
