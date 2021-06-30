use std::path::Path;

use crossterm::style::Stylize;

use super::action_result::ActionResult;
use super::printer::Printer;
use crate::runner::error::Error;
use crate::runner::RunEvent;

struct Summary {
    pub number_succeeded: u32,
    pub number_failed: u32,
}

pub struct BasicPrinter {
    display_function: Box<dyn Fn(&str)>,
    summary: Summary,
}

impl BasicPrinter {
    pub fn new() -> Self {
        Self {
            display_function: Box::new(|line: &str| println!("{}", line)),
            summary: Summary {
                number_succeeded: 0,
                number_failed: 0,
            },
        }
    }
}

impl Printer for BasicPrinter {
    fn print(&mut self, event: &RunEvent) {
        match event {
            RunEvent::SpecFileStarted(path) => self.print_spec_file(path),
            RunEvent::TestCompleted(result) => self.print_result(result),
            RunEvent::SpecFileCompleted { .. } => self.print_summary(),
            RunEvent::ErrorOccurred(error) => self.print_error(error),
        }
    }
}

impl BasicPrinter {
    fn print_spec_file(&mut self, path: &Path) {
        self.summary = Summary {
            number_succeeded: 0,
            number_failed: 0,
        };
        self.display(&format!(
            "Running tests for {}:\n",
            path.display().to_string().bold().blue()
        ));
    }

    fn print_result(&mut self, result: &ActionResult) {
        match result {
            ActionResult::Script {
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

                    format!("failed (expected exitcode {}, got {})", expected, got)
                };

                if *success {
                    self.summary.number_succeeded += 1;
                } else {
                    self.summary.number_failed += 1;
                }

                let full_message = &format!("  - script '{}' {}", name, message);
                if *success {
                    self.display_success(full_message);
                } else {
                    self.display_error(full_message);
                    self.display(&format!(
                        "\n=== stdout:\n{}\n\n=== stderr:\n{}\n\n",
                        stdout, stderr
                    ))
                }
            }
            ActionResult::Verify {
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
                    self.summary.number_succeeded += 1;
                } else {
                    self.summary.number_failed += 1;
                }

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
            ActionResult::File { path } => {
                self.summary.number_succeeded += 1;
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

    fn print_summary(&self) {
        self.display(&format!(
            "\n  {} functions run ({} succeeded / {} failed)\n",
            self.summary.number_failed + self.summary.number_succeeded,
            self.summary.number_succeeded,
            self.summary.number_failed
        ));
    }

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

#[cfg(test)]
mod tests {}
