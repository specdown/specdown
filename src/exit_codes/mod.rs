#[derive(PartialEq, Clone)]
pub enum ExitCode {
    Success = 0,
    TestFailed = 1,
    ErrorOccurred = 2,
}
