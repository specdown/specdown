use std::path::PathBuf;

use super::test_result::TestResult;
use crate::runner::Error;

pub enum PrintItem {
    SpecFileName(PathBuf),
    TestResult(TestResult),
    SpecFileSummary(),
    RunError(Error),
}

pub trait Printer {
    fn print(&mut self, item: &PrintItem);
}
