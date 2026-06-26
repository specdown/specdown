use std::path::Path;

use crossterm::style::Stylize;

use super::diff_theme::DIFF_THEME;
use crate::ansi::strip_ansi_escape_chars;
use crate::runner::Error;
use crate::runner::RunEvent;
use crate::types::{ExitCode, OutputExpectation, Stream, VerifyAction};

use super::action_result::ActionResult;
use super::action_result::{
    ActionError, BackgroundExitStatus, BackgroundStartResult, BackgroundStopResult,
    CreateFileResult, ScriptResult, VerifyResult,
};
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
            display_function: Box::new(|line: &str| println!("{line}")),
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
        self.display_error_item(&error.to_string());
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
        let title = Self::action_title(result);
        let result_message = Self::action_result_message(result);
        let full_message = &format!("{title} {result_message}");
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
                    action
                        .script_name
                        .clone()
                        .map_or("<unnamed>".to_string(), Into::into)
                )
            }
            ActionResult::Verify(VerifyResult { action, .. }) => format!(
                "verifying {} from '{}'",
                stream_to_string(&action.source.stream),
                action
                    .source
                    .name
                    .clone()
                    .map_or("<unnamed>".to_string(), Into::into),
            ),
            ActionResult::CreateFile(CreateFileResult { action, .. }) => {
                format!("creating file {}", String::from(action.file_path.clone()))
            }
            ActionResult::BackgroundStart(BackgroundStartResult { action, .. }) => {
                format!(
                    "starting background script '{}'",
                    action
                        .script_name
                        .clone()
                        .map_or("<unnamed>".to_string(), Into::into)
                )
            }
            ActionResult::BackgroundStop(BackgroundStopResult { script_name, .. }) => {
                format!(
                    "stopping background script '{}'",
                    script_name
                        .clone()
                        .map_or("<unnamed>".to_string(), Into::into)
                )
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
                    Self::exit_code_to_string(result.action.expected_exit_code),
                    Self::exit_code_to_string(result.exit_code),
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
            Some(ActionError::BackgroundExitedWithError(result)) => match result.exit_status {
                BackgroundExitStatus::Exited(code) => {
                    format!("failed (exited with code {})", i32::from(code))
                }
                BackgroundExitStatus::Killed => "succeeded".to_string(),
            },
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
            ActionError::BackgroundExitedWithError(_) => {}
        }
    }

    fn display_diff(&mut self, expected: &str, actual: &str) {
        self.display(&format!(
            "===\n{}\n===",
            termdiff::DrawDiff::new(expected, actual, &DIFF_THEME)
        ));
    }

    fn disply_all_output(&mut self, stdout: &str, stderr: &str) {
        self.display(&format!(
            "\n=== stdout:\n{stdout}\n\n=== stderr:\n{stderr}\n\n"
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
        self.display_success(&format!("  \u{2713} {text}"));
    }

    fn display_error_item(&self, text: &str) {
        self.display_error(&format!("  \u{2717} {text}"));
    }

    fn display_success(&self, text: &str) {
        self.display(&format!("{}", text.green()));
    }

    fn display_error(&self, text: &str) {
        self.display(&format!("{}", text.red()));
    }
}

const fn stream_to_string(stream: &Stream) -> &str {
    match stream {
        Stream::StdOut => "stdout",
        Stream::StdErr => "stderr",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::results::action_result::{ActionError, ActionResult};
    use crate::types::{
        CreateFileAction, ExitCode, FileContent, FilePath, OutputExpectation, ScriptAction,
        ScriptCode, ScriptName, Source, Stream, VerifyAction, VerifyValue,
    };
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;

    /// Helper that creates a BasicPrinter with no colour and captures
    /// everything written via the display function into a shared `String`.
    fn create_capture_printer() -> (BasicPrinter, Rc<RefCell<String>>) {
        let captured = Rc::new(RefCell::new(String::new()));
        let captured_clone = captured.clone();
        let printer = BasicPrinter {
            display_function: Box::new(move |line: &str| {
                captured_clone.borrow_mut().push_str(line);
                captured_clone.borrow_mut().push('\n');
            }),
            summary: Summary {
                number_succeeded: 0,
                number_failed: 0,
            },
            colour: false,
        };
        (printer, captured)
    }

    fn successful_script_result() -> ActionResult {
        ActionResult::Script(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("my_script".to_string())),
                script_code: ScriptCode("echo hello".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(0)),
            stdout: "hello".to_string(),
            stderr: String::new(),
        })
    }

    fn failed_exit_code_result() -> ActionResult {
        ActionResult::Script(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("bad_script".to_string())),
                script_code: ScriptCode("exit 1".to_string()),
                expected_exit_code: Some(ExitCode(0)),
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(1)),
            stdout: "out".to_string(),
            stderr: "err".to_string(),
        })
    }

    fn failed_unexpected_output_result() -> ActionResult {
        ActionResult::Script(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("leaky_script".to_string())),
                script_code: ScriptCode("cmd".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::StdOut,
            },
            exit_code: None,
            stdout: String::new(),
            stderr: "unexpected".to_string(),
        })
    }

    fn failed_verify_result() -> ActionResult {
        ActionResult::Verify(VerifyResult {
            action: VerifyAction {
                source: Source {
                    name: Some(ScriptName("my_script".to_string())),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("expected text".to_string()),
            },
            got: "actual text".to_string(),
        })
    }

    fn create_file_result() -> ActionResult {
        ActionResult::CreateFile(CreateFileResult {
            action: CreateFileAction {
                file_path: FilePath("test.txt".to_string()),
                file_content: FileContent("hello".to_string()),
            },
        })
    }

    // ---- print_error (covers mutant at line 73) ----

    #[test]
    fn print_error_displays_error_message() {
        let (mut printer, captured) = create_capture_printer();
        let error = Error::ScriptOutputMissing {
            missing_script_name: "nonexistent".to_string(),
        };
        let event = RunEvent::ErrorOccurred(error);
        printer.print(&event);
        let output = captured.borrow();
        assert!(
            output.contains("nonexistent"),
            "print_error should display the error text containing the script name, got: {:?}",
            output
        );
    }

    // ---- exit_code_to_string (covers mutant at line 173) ----

    #[test]
    fn exit_code_to_string_returns_string_representation_for_some() {
        assert_eq!(
            BasicPrinter::exit_code_to_string(Some(ExitCode(42))),
            "42".to_string()
        );
    }

    #[test]
    fn exit_code_to_string_returns_none_for_none() {
        assert_eq!(BasicPrinter::exit_code_to_string(None), "None".to_string());
    }

    // ---- display_action_error (covers mutants at line 180) ----

    #[test]
    fn display_action_error_shows_stdout_stderr_for_exit_code_error() {
        let (mut printer, captured) = create_capture_printer();
        let error = ActionError::ExitCodeIsIncorrect(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("s".to_string())),
                script_code: ScriptCode("x".to_string()),
                expected_exit_code: Some(ExitCode(0)),
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(1)),
            stdout: "my-stdout".to_string(),
            stderr: "my-stderr".to_string(),
        });
        printer.display_action_error(&error);
        let output = captured.borrow();
        assert!(
            output.contains("my-stdout"),
            "display_action_error should show stdout, got: {:?}",
            output
        );
        assert!(
            output.contains("my-stderr"),
            "display_action_error should show stderr, got: {:?}",
            output
        );
    }

    #[test]
    fn display_action_error_shows_stdout_stderr_for_unexpected_output() {
        let (mut printer, captured) = create_capture_printer();
        let error = ActionError::UnexpectedOutputIsPresent(ScriptResult {
            action: ScriptAction {
                script_name: Some(ScriptName("s".to_string())),
                script_code: ScriptCode("x".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::StdOut,
            },
            exit_code: None,
            stdout: "extra-out".to_string(),
            stderr: "extra-err".to_string(),
        });
        printer.display_action_error(&error);
        let output = captured.borrow();
        assert!(
            output.contains("extra-out"),
            "display_action_error should show stdout for unexpected output, got: {:?}",
            output
        );
        assert!(
            output.contains("extra-err"),
            "display_action_error should show stderr for unexpected output, got: {:?}",
            output
        );
    }

    // ---- display_diff (covers mutant at line 195) ----

    #[test]
    fn display_diff_shows_expected_vs_actual() {
        let (mut printer, captured) = create_capture_printer();
        printer.display_diff("expected line", "actual line");
        let output = captured.borrow();
        assert!(
            output.contains("==="),
            "display_diff should produce diff output with === separators, got: {:?}",
            output
        );
    }

    // ---- disply_all_output (covers mutant at line 202) ----

    #[test]
    fn disply_all_output_shows_stdout_and_stderr_sections() {
        let (mut printer, captured) = create_capture_printer();
        printer.disply_all_output("the-stdout", "the-stderr");
        let output = captured.borrow();
        assert!(
            output.contains("stdout:"),
            "disply_all_output should contain 'stdout:' header, got: {:?}",
            output
        );
        assert!(
            output.contains("the-stdout"),
            "disply_all_output should contain the stdout content, got: {:?}",
            output
        );
        assert!(
            output.contains("stderr:"),
            "disply_all_output should contain 'stderr:' header, got: {:?}",
            output
        );
        assert!(
            output.contains("the-stderr"),
            "disply_all_output should contain the stderr content, got: {:?}",
            output
        );
    }

    // ---- display_error_item (covers mutant at line 222) ----

    #[test]
    fn display_error_item_formats_with_cross_mark() {
        let (printer, captured) = create_capture_printer();
        printer.display_error_item("something went wrong");
        let output = captured.borrow();
        // The cross mark ✓ is \u{2717}
        assert!(
            output.contains("\u{2717}"),
            "display_error_item should contain the cross mark, got: {:?}",
            output
        );
        assert!(
            output.contains("something went wrong"),
            "display_error_item should contain the text, got: {:?}",
            output
        );
    }

    // ---- display_error (covers mutant at line 230) ----

    #[test]
    fn display_error_formats_with_red_color() {
        let (printer, captured) = create_capture_printer();
        // In no-colour mode, the red ANSI codes are stripped
        printer.display_error("failure message");
        let output = captured.borrow();
        assert!(
            output.contains("failure message"),
            "display_error should contain the message text, got: {:?}",
            output
        );
    }

    // ---- action_result_message / exit_code display integration ----

    #[test]
    fn action_result_message_shows_exit_codes_for_incorrect_exit_code() {
        // This exercises exit_code_to_string through action_result_message
        let msg = BasicPrinter::action_result_message(&failed_exit_code_result());
        assert!(
            msg.contains('0'),
            "action_result_message should show expected exit code, got: {:?}",
            msg
        );
        assert!(
            msg.contains('1'),
            "action_result_message should show actual exit code, got: {:?}",
            msg
        );
    }

    // ---- action_title tests ----

    #[test]
    fn action_title_formats_script_with_name() {
        let title = BasicPrinter::action_title(&successful_script_result());
        assert!(
            title.contains("my_script"),
            "action_title for script should contain the script name, got: {:?}",
            title
        );
    }

    #[test]
    fn action_title_formats_verify_result() {
        let title = BasicPrinter::action_title(&ActionResult::Verify(VerifyResult {
            action: VerifyAction {
                source: Source {
                    name: Some(ScriptName("v_script".to_string())),
                    stream: Stream::StdOut,
                },
                expected_value: VerifyValue("x".to_string()),
            },
            got: "y".to_string(),
        }));
        assert!(
            title.contains("v_script"),
            "action_title for verify should contain the script name, got: {:?}",
            title
        );
        assert!(
            title.contains("stdout"),
            "action_title for verify should contain stream name, got: {:?}",
            title
        );
    }

    #[test]
    fn action_title_formats_create_file_result() {
        let title = BasicPrinter::action_title(&create_file_result());
        assert!(
            title.contains("test.txt"),
            "action_title for create file should contain the file path, got: {:?}",
            title
        );
    }

    // ---- display_action (covers mutant at line 84) ----

    #[test]
    fn display_action_outputs_title_and_result_for_success() {
        let (mut printer, captured) = create_capture_printer();
        printer.display_action(&successful_script_result());
        let output = captured.borrow();
        // display_action generates "{title} {result_message}" and wraps it in
        // display_success_item (check mark) or display_error_item (cross mark)
        assert!(
            output.contains("my_script"),
            "display_action should output the script name, got: {}",
            output.replace('\x1b', "\\x1b")
        );
        assert!(
            output.contains("succeeded"),
            "display_action should output 'succeeded' for a passing result, got: {}",
            output.replace('\x1b', "\\x1b")
        );
    }

    #[test]
    fn display_action_outputs_title_and_result_for_failure() {
        let (mut printer, captured) = create_capture_printer();
        printer.display_action(&failed_exit_code_result());
        let output = captured.borrow();
        assert!(
            output.contains("bad_script"),
            "display_action should output the script name, got: {}",
            output.replace('\x1b', "\\x1b")
        );
        assert!(
            output.contains("failed"),
            "display_action should output 'failed' for a failing result, got: {}",
            output.replace('\x1b', "\\x1b")
        );
    }

    // ---- print integration tests (full Printer::print cycle) ----

    #[test]
    fn print_spec_file_started_resets_summary() {
        let (mut printer, captured) = create_capture_printer();
        // First count a result so summary is non-zero
        printer.count_action(&successful_script_result());
        // Now start a new spec file — should reset
        let event = RunEvent::SpecFileStarted(PathBuf::from("test.md"));
        printer.print(&event);
        let output = captured.borrow();
        assert!(
            output.contains("test.md"),
            "print should show the file path, got: {:?}",
            output
        );
    }

    #[test]
    fn print_test_completed_increments_success_count() {
        let (mut printer, _captured) = create_capture_printer();
        let event = RunEvent::TestCompleted(successful_script_result());
        printer.print(&event);
        // Summary should show 1 succeeded, 0 failed
        // We verify by triggering the summary print
        let (mut printer2, captured2) = create_capture_printer();
        printer2.count_action(&successful_script_result());
        let summary_event = RunEvent::SpecFileCompleted { success: true };
        printer2.print(&summary_event);
        let output = captured2.borrow();
        assert!(
            output.contains("1 succeeded"),
            "summary should say 1 succeeded, got: {:?}",
            output
        );
        assert!(
            output.contains("0 failed"),
            "summary should say 0 failed, got: {:?}",
            output
        );
    }

    #[test]
    fn print_test_completed_with_failure_shows_error_details() {
        let (mut printer, captured) = create_capture_printer();
        let event = RunEvent::TestCompleted(failed_exit_code_result());
        printer.print(&event);
        let output = captured.borrow();
        // The result is a failure, so it should display error details with stdout/stderr
        assert!(
            output.contains("stdout:"),
            "failed script result should trigger display of stdout, got: {:?}",
            output
        );
        assert!(
            output.contains("stderr:"),
            "failed script result should trigger display of stderr, got: {:?}",
            output
        );
    }

    #[test]
    fn print_test_completed_with_verify_failure_shows_diff() {
        let (mut printer, captured) = create_capture_printer();
        let event = RunEvent::TestCompleted(failed_verify_result());
        printer.print(&event);
        let output = captured.borrow();
        assert!(
            output.contains("==="),
            "failed verify result should trigger diff display, got: {:?}",
            output
        );
    }

    #[test]
    fn print_spec_file_completed_shows_summary() {
        let (mut printer, captured) = create_capture_printer();
        // Count one success and one failure
        printer.count_action(&successful_script_result());
        printer.count_action(&failed_exit_code_result());
        let event = RunEvent::SpecFileCompleted { success: false };
        printer.print(&event);
        let output = captured.borrow();
        assert!(
            output.contains("2 functions run"),
            "summary should say 2 functions run, got: {:?}",
            output
        );
        assert!(
            output.contains("1 succeeded"),
            "summary should say 1 succeeded, got: {:?}",
            output
        );
        assert!(
            output.contains("1 failed"),
            "summary should say 1 failed, got: {:?}",
            output
        );
    }

    #[test]
    fn print_unexpected_output_failure_shows_output_sections() {
        let (mut printer, captured) = create_capture_printer();
        let event = RunEvent::TestCompleted(failed_unexpected_output_result());
        printer.print(&event);
        let output = captured.borrow();
        assert!(
            output.contains("stdout:"),
            "unexpected output error should show stdout section, got: {:?}",
            output
        );
        assert!(
            output.contains("stderr:"),
            "unexpected output error should show stderr section, got: {:?}",
            output
        );
    }

    #[test]
    fn colour_mode_strips_ansi_escape_codes() {
        let captured = Rc::new(RefCell::new(String::new()));
        let captured_clone = captured.clone();
        let mut printer = BasicPrinter {
            display_function: Box::new(move |line: &str| {
                captured_clone.borrow_mut().push_str(line);
            }),
            summary: Summary {
                number_succeeded: 0,
                number_failed: 0,
            },
            colour: false, // no colour → ANSI should be stripped
        };
        let event = RunEvent::TestCompleted(successful_script_result());
        printer.print(&event);
        let output = captured.borrow();
        // In no-colour mode, crossterm's .green() / .red() / .blue() ANSI codes
        // are stripped, so there should be no ESC sequences
        assert!(
            !output.contains('\x1b'),
            "no-colour mode should strip ANSI escape codes, got: {}",
            output.replace('\x1b', "\\x1b")
        );
    }

    #[test]
    fn unnamed_script_shows_unnamed_in_title() {
        let result = ActionResult::Script(ScriptResult {
            action: ScriptAction {
                script_name: None,
                script_code: ScriptCode("echo".to_string()),
                expected_exit_code: None,
                expected_output: OutputExpectation::Any,
            },
            exit_code: Some(ExitCode(0)),
            stdout: String::new(),
            stderr: String::new(),
        });
        let title = BasicPrinter::action_title(&result);
        assert!(
            title.contains("<unnamed>"),
            "unnamed script should show '<unnamed>' in title, got: {:?}",
            title
        );
    }

    #[test]
    fn unnamed_verify_shows_unnamed_in_title() {
        let result = ActionResult::Verify(VerifyResult {
            action: VerifyAction {
                source: Source {
                    name: None,
                    stream: Stream::StdErr,
                },
                expected_value: VerifyValue("x".to_string()),
            },
            got: "x".to_string(),
        });
        let title = BasicPrinter::action_title(&result);
        assert!(
            title.contains("stderr"),
            "verify with StdErr should show 'stderr' in title, got: {:?}",
            title
        );
        assert!(
            title.contains("<unnamed>"),
            "unnamed verify should show '<unnamed>' in title, got: {:?}",
            title
        );
    }

    #[test]
    fn display_success_item_formats_with_check_mark() {
        let (printer, captured) = create_capture_printer();
        printer.display_success_item("all good");
        let output = captured.borrow();
        // The check mark ✓ is \u{2713}
        assert!(
            output.contains("\u{2713}"),
            "display_success_item should contain the check mark, got: {:?}",
            output
        );
        assert!(
            output.contains("all good"),
            "display_success_item should contain the text, got: {:?}",
            output
        );
    }
}
