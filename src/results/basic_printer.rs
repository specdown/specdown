use std::path::Path;

use crossterm::style::Stylize;

use crate::ansi::strip_ansi_escape_chars;
use crate::results::action_result::{ActionError, CreateFileResult, ScriptResult, VerifyResult};
use crate::runner::Error;
use crate::runner::RunEvent;
use crate::types::{ExitCode, OutputExpectation, Stream, VerifyAction};

use super::action_result::ActionResult;
use super::printer::Printer;

struct Summary {
    pub number_succeeded: u32,
    pub number_failed: u32,
}

pub struct BasicPrinter {
    display_function: Box<dyn Fn(&str)>,
    summary: Summary,
    colour: bool,
}

impl BasicPrinter {
    pub fn new(colour: bool) -> Self {
        Self {
            display_function: Box::new(|line: &str| println!("{}", line)),
            summary: Summary {
                number_succeeded: 0,
                number_failed: 0,
            },
            colour,
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
        if let Some(error) = result.error() {
            self.display_action_error(&error);
        }
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
            ActionResult::Script(ScriptResult { action, .. }) => {
                format!(
                    "running script '{}'",
                    String::from(action.script_name.clone())
                )
            }
            ActionResult::Verify(VerifyResult { action, .. }) => format!(
                "verifying {} from '{}'",
                stream_to_string(&action.source.stream),
                String::from(action.source.name.clone()),
            ),
            ActionResult::CreateFile(CreateFileResult { action, .. }) => {
                format!("creating file {}", String::from(action.file_path.clone()))
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
        match result.error() {
            Some(ActionError::ExitCodeIsIncorrect(result)) => {
                format!(
                    "failed (expected exitcode {}, got {})",
                    BasicPrinter::exit_code_to_string(result.action.expected_exit_code),
                    BasicPrinter::exit_code_to_string(result.exit_code),
                )
            }
            Some(ActionError::UnexpectedOutputIsPresent(result)) => {
                format!(
                    "failed (unexpected {})",
                    match result.action.expected_output {
                        OutputExpectation::Any => panic!("Should not be possible"),
                        OutputExpectation::StdOut => "stderr",
                        OutputExpectation::StdErr => "stdout",
                        OutputExpectation::None => "output",
                    }
                )
            }
            Some(ActionError::OutputDoesNotMatch(_)) => "failed".to_string(),
            None => "succeeded".to_string(),
        }
    }

    fn exit_code_to_string(exit_code: Option<ExitCode>) -> String {
        exit_code
            .map(String::from)
            .or_else(|| Some("None".to_string()))
            .unwrap()
    }

    fn display_action_error(&mut self, error: &ActionError) {
        match error {
            ActionError::ExitCodeIsIncorrect(ScriptResult { stdout, stderr, .. })
            | ActionError::UnexpectedOutputIsPresent(ScriptResult { stdout, stderr, .. }) => {
                self.disply_all_output(stdout, stderr);
            }
            ActionError::OutputDoesNotMatch(VerifyResult {
                action: VerifyAction { expected_value, .. },
                got,
            }) => {
                self.display_diff(&String::from(expected_value.clone()), got);
            }
        }
    }

    fn display_diff(&mut self, expected: &str, actual: &str) {
        self.display(&format!(
            "===\n{}\n===",
            colored_diff::PrettyDifference { expected, actual }
        ));
    }

    fn disply_all_output(&mut self, stdout: &str, stderr: &str) {
        self.display(&format!(
            "\n=== stdout:\n{}\n\n=== stderr:\n{}\n\n",
            stdout, stderr
        ));
    }

    fn display(&self, text: &str) {
        let display = &self.display_function;
        let prepared_text = if self.colour {
            text.to_string()
        } else {
            strip_ansi_escape_chars(text)
        };
        display(&prepared_text);
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
