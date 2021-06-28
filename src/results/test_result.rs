use crate::runner::Summary;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub enum TestResult {
    Script {
        name: String,
        exit_code: Option<i32>,
        expected_exit_code: Option<i32>,
        script: String,
        stdout: String,
        stderr: String,
        success: bool,
    },
    Verify {
        script_name: String,
        stream: String,
        expected: String,
        got: String,
        success: bool,
    },
    File {
        path: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpecResult {
    // TODO: Fix access and move
    pub(crate) file_name: PathBuf,
    pub(crate) results: Vec<TestResult>,
    pub(crate) summary: Summary,
    pub(crate) success: bool,
}
