use std::path::PathBuf;

use super::test_result::TestResult;
use crate::runner::{Error, Summary};

pub enum PrintItem {
    SpecFileName(PathBuf),
    TestResult(TestResult),
    SpecFileSummary(Summary),
    RunError(Error),
}

pub trait Printer {
    fn print(&self, item: &PrintItem);
}
