#[derive(Clone, Debug, PartialEq)]
pub enum TestResult {
    Script {
        name: String,
        exit_code: u8,
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
