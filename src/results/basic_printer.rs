use std::path::Path;

use crossterm::style::Stylize;

use super::action_result::ActionResult;
use super::printer::Printer;
use crate::runner::error::Error;
use crate::runner::RunEvent;
use crate::types::{ScriptAction, Stream, VerifyAction};

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
        self.count_action(result);
        self.display_action(result);
        self.dsiplay_action_error(result);
    }

    fn print_error(&self, error: &Error) {
        self.display_error_item(&match error {
            Error::ScriptOutputMissing {
                missing_script_name,
            } => {
                format!(
                    "Failed to verify the output of '{}': No script with that name has been executed yet.",
                    missing_script_name
                )
            },
            Error::CommandFailed { command, message } => format!(
                "Failed to run command: {} (Error: {})",
                command, message
            ),
            Error::BadShellCommand { command, message } => format!(
                "Invalid shell command provided: {} (Error: {})",
                command, message
            ),
            Error::RunFailed { message } => message.to_string(),
        });
    }

    fn print_summary(&self) {
        self.display(&format!(
            "\n  {} functions run ({} succeeded / {} failed)\n",
            self.summary.number_failed + self.summary.number_succeeded,
            self.summary.number_succeeded,
            self.summary.number_failed
        ));
    }

    fn display_action(&mut self, result: &ActionResult) {
        let title = BasicPrinter::action_title(result);
        let result_message = BasicPrinter::action_result_message(result);
        let full_message = &format!("{} {}", title, result_message);
        if result.success() {
            self.display_success_item(full_message);
        } else {
            self.display_error_item(full_message);
        }
    }

    fn action_title(result: &ActionResult) -> String {
        match result {
            ActionResult::Script { action, .. } => {
                format!("script '{}'", String::from(action.script_name.clone()))
            }
            ActionResult::Verify { action, .. } => format!(
                "verify {} from '{}'",
                stream_to_string(&action.source.stream),
                String::from(action.source.name.clone()),
            ),
            ActionResult::CreateFile { action, .. } => {
                format!("file {}", String::from(action.file_path.clone()))
            }
        }
    }

    fn count_action(&mut self, result: &ActionResult) {
        if result.success() {
            self.summary.number_succeeded += 1;
        } else {
            self.summary.number_failed += 1;
        }
    }

    fn action_result_message(result: &ActionResult) -> String {
        match result {
            ActionResult::Script {
                action: ScriptAction {
                    expected_exit_code, ..
                },
                exit_code,
                ..
            } => {
                if result.success() {
                    "succeeded".to_string()
                } else {
                    let expected =
                        expected_exit_code.map_or("None".to_string(), |code| code.into());
                    let got = exit_code.map_or("None".to_string(), |code| code.to_string());

                    format!("failed (expected exitcode {}, got {})", expected, got)
                }
            }
            ActionResult::Verify { .. } => {
                if result.success() {
                    "succeeded".to_string()
                } else {
                    "failed".to_string()
                }
            }
            ActionResult::CreateFile { .. } => "created".to_string(),
        }
    }

    fn dsiplay_action_error(&mut self, result: &ActionResult) {
        match result {
            ActionResult::Script { stdout, stderr, .. } => {
                if !result.success() {
                    self.display(&format!(
                        "\n=== stdout:\n{}\n\n=== stderr:\n{}\n\n",
                        stdout, stderr
                    ));
                }
            }
            ActionResult::Verify { action, got, .. } => {
                let VerifyAction {
                    expected_value: expected,
                    ..
                } = action;

                if !result.success() {
                    self.display(&format!(
                        "===\n{}\n===",
                        colored_diff::PrettyDifference {
                            expected: &String::from(expected.clone()),
                            actual: got,
                        }
                    ));
                }
            }
            ActionResult::CreateFile { .. } => {}
        }
    }

    fn display(&self, text: &str) {
        let display = &self.display_function;
        display(text);
    }

    fn display_success_item(&self, text: &str) {
        self.display_success(&format!("  - {}", text));
    }

    fn display_error_item(&self, text: &str) {
        self.display_error(&format!("  - {}", text));
    }

    fn display_success(&self, text: &str) {
        self.display(&format!("{}", text.green()));
    }

    fn display_error(&self, text: &str) {
        self.display(&format!("{}", text.red()));
    }
}

fn stream_to_string(stream: &Stream) -> &str {
    match stream {
        Stream::StdOut => "stdout",
        Stream::StdErr => "stderr",
    }
}

#[cfg(test)]
mod tests {}
