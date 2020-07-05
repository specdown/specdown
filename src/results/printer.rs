use std::path::Path;

use super::test_result::TestResult;
use crate::runner::{Error, Summary};

pub trait Printer {
    fn print_spec_file(&self, path: &Path);
    fn print_result(&self, result: &TestResult);
    fn print_error(&self, error: &Error);
    fn print_summary(&self, summary: &Summary);
}
