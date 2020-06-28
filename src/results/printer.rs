use super::test_result::TestResult;
use crate::runner::Error;

pub trait Printer {
    fn print_result(&self, result: &TestResult);
    fn print_error(&self, error: &Error);
}
