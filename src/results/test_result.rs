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
