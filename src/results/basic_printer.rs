use std::path::Path;

use crossterm::style::Stylize;

use super::printer::Printer;
use super::test_result::TestResult;
use crate::runner::{Error, Summary};

pub struct BasicPrinter {
    display_function: Box<dyn Fn(&str)>,
}

impl BasicPrinter {
    pub fn new() -> Self {
        Self {
            display_function: Box::new(|line: &str| println!("{}", line)),
        }
    }
}

impl BasicPrinter {
    fn display(&self, text: &str) {
        let display = &self.display_function;
        display(text);
    }

    fn display_success(&self, text: &str) {
        self.display(&format!("{}", text.green()));
    }

    fn display_error(&self, text: &str) {
        self.display(&format!("{}", text.red()));
    }
}

impl Printer for BasicPrinter {
    fn print_spec_file(&self, path: &Path) {
        self.display(&format!(
            "Running tests for {}:\n",
            path.display().to_string().bold().blue()
        ));
    }

    fn print_result(&self, result: &TestResult) {
        match result {
            TestResult::Script {
                name,
                success,
                expected_exit_code,
                exit_code,
                stdout,
                stderr,
                ..
            } => {
                let message = if *success {
                    "succeeded".to_string()
                } else {
                    let expected =
                        expected_exit_code.map_or("None".to_string(), |code| code.to_string());
                    let got = exit_code.map_or("None".to_string(), |code| code.to_string());

                    format!(
                        "failed (expected exitcode {}, got {})\n=== stdout:\n{}\n\n=== stderr:\n{}\n\n",
                        expected, got, stdout, stderr
                    )
                };

                let full_message = &format!("  - script '{}' {}", name, message);
                if *success {
                    self.display_success(full_message);
                } else {
                    self.display_error(full_message);
                }
            }
            TestResult::Verify {
                script_name,
                success,
                expected,
                got,
                stream,
            } => {
                let message = &format!(
                    "  - verify {} from '{}' {}",
                    stream,
                    script_name,
                    if *success { "succeeded" } else { "failed" }
                );

                if *success {
                    self.display_success(message);
                } else {
                    self.display_error(message);
                    self.display(&format!(
                        "===\n{}\n===",
                        colored_diff::PrettyDifference {
                            expected,
                            actual: got
                        }
                    ));
                }
            }
            TestResult::File { path } => {
                self.display_success(&format!("  - file {} created", path));
            }
        }
    }

    fn print_error(&self, error: &Error) {
        match error {
            Error::ScriptOutputMissing {
                missing_script_name,
            } => {
                self.display_error(&format!(
                    "  - Failed to verify the output of '{}': No script with that name has been executed yet.",
                    missing_script_name
                ));
            }
            Error::CommandFailed { command, message } => self.display_error(&format!(
                "  - Failed to run command: {} (Error: {})",
                command, message
            )),
            Error::BadShellCommand { command, message } => self.display_error(&format!(
                "  - Invalid shell command provided: {} (Error: {})",
                command, message
            )),
            Error::RunFailed { message } => self.display_error(&format!("  - {}", message)),
        }
    }

    fn print_summary(&self, summary: &Summary) {
        self.display(&format!(
            "\n  {} functions run ({} succeeded / {} failed)",
            summary.number_failed + summary.number_succeeded,
            summary.number_succeeded,
            summary.number_failed
        ));
    }
}

#[cfg(test)]
mod tests {}
