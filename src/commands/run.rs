use clap::{Arg, SubCommand};
use std::fs;
use std::path::{Path, PathBuf};

use crate::exit_codes::ExitCode;
use crate::parser;
use crate::results::basic_printer::BasicPrinter;
use crate::results::printer::{PrintItem, Printer};
use crate::results::test_result::SpecResult;
use crate::runner::{run_actions, Error, RunEvent};

pub const NAME: &str = "run";

pub fn create() -> clap::App<'static, 'static> {
    let spec_file = Arg::with_name("spec-files")
        .index(1)
        .min_values(1)
        .help("The spec files to run")
        .required(true);

    let test_dir = Arg::with_name("running-dir")
        .long("running-dir")
        .takes_value(true)
        .help("The directory where commands will be executed")
        .required(false);

    let shell_cmd = Arg::with_name("shell-command")
        .long("shell-command")
        .takes_value(true)
        .default_value("bash -c")
        .help("The shell command used to execute script blocks")
        .required(false);

    SubCommand::with_name(NAME)
        .about("Runs a given Markdown Specification")
        .arg(spec_file)
        .arg(test_dir)
        .arg(shell_cmd)
}

pub fn execute(run_matches: &clap::ArgMatches<'_>) {
    let spec_files = run_matches
        .values_of("spec-files")
        .expect("spec-files should always exist")
        .map(Path::new)
        .map(std::path::Path::to_path_buf)
        .collect();
    let running_dir = run_matches
        .value_of("running-dir")
        .map(Path::new)
        .map(std::path::Path::to_path_buf);
    let shell_cmd = run_matches.value_of("shell-command").unwrap().to_string();
    let spec_dir = std::env::current_dir().expect("Failed to get current working directory");
    let printer = Box::new(BasicPrinter::new());

    let mut command = RunCommand {
        spec_files,
        spec_dir,
        shell_cmd,
        running_dir,
        printer,
    };

    command.execute();
}

struct RunCommand {
    spec_files: Vec<PathBuf>,
    spec_dir: PathBuf,
    shell_cmd: String,
    running_dir: Option<PathBuf>,
    printer: Box<dyn Printer>,
}

impl RunCommand {
    pub fn execute(&mut self) {
        self.change_to_running_directory();

        let spec_files = self.spec_files.clone();

        for spec_file in spec_files {
            let (exit_code, print_items) = self.run_spec_file(&spec_file);
            self.print_items(print_items);
            if exit_code != ExitCode::Success {
                std::process::exit(exit_code as i32)
            }
        }
    }

    fn run_spec_file(&self, spec_file: &Path) -> (ExitCode, Vec<PrintItem>) {
        let contents = self.read_file(spec_file);
        let result = parser::parse(&contents)
            .map_err(|err| {
                let error = Error::RunFailed {
                    message: err.to_string(),
                };
                (
                    ExitCode::TestFailed, // TODO: Fix exit code, change would make BC break
                    error,
                )
            })
            .and_then(|action_list| {
                run_actions(spec_file, &action_list, &self.shell_cmd)
                    .map_err(|err| (ExitCode::ErrorOccurred, err))
            })
            .map(|events| RunCommand::events_to_result(&events));

        (
            RunCommand::exit_code(&result),
            RunCommand::create_print_items(spec_file, &result),
        )
    }

    fn events_to_result(events: &[RunEvent]) -> SpecResult {
        let mut file_name: Option<PathBuf> = None;
        let mut results = Vec::new();
        let mut the_success: Option<bool> = None;

        for event in events {
            match event {
                RunEvent::SpecFileStarted(name) => file_name = Some(name.clone()),
                RunEvent::TestCompleted(result) => results.push(result.clone()),
                RunEvent::SpecFileCompleted { success } => {
                    the_success = Some(*success);
                }
            }
        }

        SpecResult {
            file_name: file_name.expect("expected: file_name"),
            results,
            success: the_success.expect("expected: success"),
        }
    }

    fn create_print_items(
        spec_file: &Path,
        result: &Result<SpecResult, (ExitCode, Error)>,
    ) -> Vec<PrintItem> {
        let mut print_items = match &result {
            Ok(SpecResult { results, .. }) => {
                let mut print_items = results
                    .iter()
                    .map(|result| PrintItem::TestResult(result.clone()))
                    .collect::<Vec<PrintItem>>();
                print_items.push(PrintItem::SpecFileSummary());
                print_items
            }
            Err((_, err)) => vec![PrintItem::RunError(err.clone())],
        };

        let mut all_print_items = vec![PrintItem::SpecFileName(spec_file.to_path_buf())];
        all_print_items.append(&mut print_items);
        all_print_items
    }

    fn exit_code(result: &Result<SpecResult, (ExitCode, Error)>) -> ExitCode {
        match &result {
            Ok(SpecResult { success, .. }) => {
                if *success {
                    ExitCode::Success
                } else {
                    ExitCode::TestFailed
                }
            }
            Err((exit_code, _)) => exit_code.clone(),
        }
    }

    fn read_file(&self, spec_file: &Path) -> String {
        fs::read_to_string(self.to_absolute(spec_file)).expect("failed to read spec file")
    }

    fn change_to_running_directory(&self) {
        if let Some(dir) = &self.running_dir {
            fs::create_dir_all(dir).expect("Failed to create running directory");
            std::env::set_current_dir(dir).expect("Failed to set running directory");
        }
    }

    pub fn to_absolute(&self, path: &Path) -> PathBuf {
        if path.has_root() {
            path.to_path_buf()
        } else {
            self.spec_dir.join(path)
        }
    }

    fn print_items(&mut self, items: Vec<PrintItem>) {
        for item in items {
            self.printer.print(&item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RunCommand;

    mod to_absolute {
        use super::RunCommand;
        use crate::results::basic_printer::BasicPrinter;
        use std::path::Path;

        fn command() -> RunCommand {
            RunCommand {
                spec_files: vec![],
                spec_dir: Path::new("/usr/local/specdown").to_path_buf(),
                shell_cmd: "".to_string(),
                running_dir: None,
                printer: Box::new(BasicPrinter::new()),
            }
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_path_when_it_is_absolute() {
            let path = Path::new("/home/user/file");
            assert_eq!(path, command().to_absolute(path));
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_working_dir_prepended_when_path_is_relative() {
            let path = Path::new("./file");
            assert_eq!(
                Path::new("/usr/local/specdown/file"),
                command().to_absolute(path)
            );
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_working_dir_prepended_when_path_contains_parent() {
            let path = Path::new("../file");
            assert_eq!(
                Path::new("/usr/local/specdown/../file"),
                command().to_absolute(path)
            );
        }
    }
}
