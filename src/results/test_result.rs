pub enum TestResult {
    ScriptResult {
        name: String,
        exit_code: u8,
        script: String,
        output: String,
        stdout: String,
        stderr: String,
        success: bool,
    },
    VerifyResult {
        script_name: String,
        stream: String,
        expected: String,
        got: String,
        success: bool,
    },
}
