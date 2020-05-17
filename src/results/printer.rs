use super::test_result::TestResult;

pub trait Printer {
    fn print(&self, result: &TestResult);
}
