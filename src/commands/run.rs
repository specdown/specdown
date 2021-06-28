use clap::{Arg, SubCommand};
use std::fs;
use std::path::{Path, PathBuf};

use crate::exit_codes::ExitCode;
use crate::parser;
use crate::results::basic_printer::BasicPrinter;
use crate::results::printer::Printer;
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
    let mut printer = Box::new(BasicPrinter::new());

    let mut command = RunCommand {
        spec_files,
        spec_dir,
        shell_cmd,
        running_dir,
    };

    let (events, exit_code) = command.execute();

    for event in events {
        printer.print(&event);
    }

    std::process::exit(exit_code as i32)
}

struct RunCommand {
    spec_files: Vec<PathBuf>,
    spec_dir: PathBuf,
    shell_cmd: String,
    running_dir: Option<PathBuf>,
}

impl RunCommand {
    pub fn execute(&mut self) -> (Vec<RunEvent>, ExitCode) {
        self.change_to_running_directory();

        let spec_files = self.spec_files.clone();

        let mut all_events = Vec::new();

        for spec_file in spec_files {
            let (exit_code, mut events) = self.run_spec_file(&spec_file);
            all_events.append(&mut events);
            if exit_code != ExitCode::Success {
                return (all_events, exit_code);
            }
        }

        (all_events, ExitCode::Success)
    }

    fn run_spec_file(&self, spec_file: &Path) -> (ExitCode, Vec<RunEvent>) {
        let contents = self.read_file(spec_file);
        let events = parser::parse(&contents)
            .map_err(|err| Error::RunFailed {
                message: err.to_string(),
            })
            .map(|action_list| run_actions(spec_file, &action_list, &self.shell_cmd))
            .or_else::<Error, _>(|err| {
                Ok(vec![
                    RunEvent::SpecFileStarted(spec_file.to_path_buf()),
                    RunEvent::ErrorOccurred(err),
                ])
            })
            .unwrap();

        let result = RunCommand::events_to_exit_code(&events);

        (result, events)
    }

    fn events_to_exit_code(events: &[RunEvent]) -> ExitCode {
        let mut exit_code = ExitCode::Success;

        for event in events {
            match event {
                RunEvent::SpecFileCompleted { success: false } => {
                    if exit_code == ExitCode::Success {
                        exit_code = ExitCode::TestFailed;
                    }
                }
                RunEvent::ErrorOccurred(error) => {
                    return match error {
                        Error::RunFailed { .. } => ExitCode::TestFailed,
                        _ => ExitCode::ErrorOccurred,
                    }
                }
                _ => {}
            }
        }

        exit_code
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
}

#[cfg(test)]
mod tests {
    use super::RunCommand;

    mod to_absolute {
        use super::RunCommand;
        use std::path::Path;

        fn command() -> RunCommand {
            RunCommand {
                spec_files: vec![],
                spec_dir: Path::new("/usr/local/specdown").to_path_buf(),
                shell_cmd: "".to_string(),
                running_dir: None,
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
