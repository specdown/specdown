use clap::{Arg, SubCommand};
use std::fs;
use std::path::{Path, PathBuf};

use crate::exit_codes::ExitCode;
use crate::parser;
use crate::results::basic_printer::BasicPrinter;
use crate::results::printer::Printer;
use crate::runner::error::Error;
use crate::runner::executor::{Executor, Shell};
use crate::runner::state::State;
use crate::runner::{runnable_action, RunEvent};
use crate::types::Action;

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
    let file_reader = FileReader { dir: spec_dir };
    let mut printer = Box::new(BasicPrinter::new());

    let events = match Shell::new(&shell_cmd) {
        Ok(executor) => RunCommand {
            spec_files,
            executor: Box::new(executor),
            running_dir,
            file_reader,
        }
        .execute(),
        Err(err) => vec![RunEvent::ErrorOccurred(err)],
    };

    for event in &events {
        printer.print(event);
    }

    let exit_code = events_to_exit_code(&events);

    std::process::exit(exit_code as i32)
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

struct FileReader {
    dir: PathBuf,
}

impl FileReader {
    fn read_file(&self, spec_file: &Path) -> String {
        fs::read_to_string(self.to_absolute(spec_file)).expect("failed to read spec file")
    }

    pub fn to_absolute(&self, path: &Path) -> PathBuf {
        if path.has_root() {
            path.to_path_buf()
        } else {
            self.dir.join(path)
        }
    }
}

struct RunCommand {
    spec_files: Vec<PathBuf>,
    executor: Box<dyn Executor>,
    running_dir: Option<PathBuf>,
    file_reader: FileReader,
}

impl RunCommand {
    pub fn execute(&self) -> Vec<RunEvent> {
        self.change_to_running_directory();

        self.spec_files
            .iter()
            .flat_map(|spec_file| self.run_spec_file(&spec_file))
            .collect()
    }

    fn run_spec_file(&self, spec_file: &Path) -> Vec<RunEvent> {
        let mut state = State::new();

        let start_events = vec![RunEvent::SpecFileStarted(spec_file.to_path_buf())];
        let contents = self.file_reader.read_file(spec_file);
        let run_events = parser::parse(&contents)
            .map_err(|err| Error::RunFailed {
                message: err.to_string(),
            })
            .map(|action_list| self.run_actions(&mut state, &action_list))
            .or_else::<Error, _>(|err| Ok(vec![RunEvent::ErrorOccurred(err)]))
            .unwrap();
        let end_events = vec![RunEvent::SpecFileCompleted {
            success: state.is_success(),
        }];

        start_events
            .into_iter()
            .chain(run_events.into_iter())
            .chain(end_events.into_iter())
            .collect()
    }

    fn run_actions(&self, mut state: &mut State, actions: &[Action]) -> Vec<RunEvent> {
        actions
            .iter()
            .map(|action| self.run_single_action(&mut state, action))
            .collect()
    }

    fn run_single_action(&self, state: &mut State, action: &Action) -> RunEvent {
        runnable_action::from_action(action)
            .run(&state, &*self.executor)
            .map(|result| {
                state.add_result(&result);
                RunEvent::TestCompleted(result)
            })
            .or_else::<Error, _>(|error| Ok(RunEvent::ErrorOccurred(error)))
            .unwrap()
    }

    fn change_to_running_directory(&self) {
        if let Some(dir) = &self.running_dir {
            fs::create_dir_all(dir).expect("Failed to create running directory");
            std::env::set_current_dir(dir).expect("Failed to set running directory");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileReader;

    mod to_absolute {
        use super::FileReader;
        use std::path::Path;

        fn reader() -> FileReader {
            FileReader {
                dir: Path::new("/usr/local/specdown").to_path_buf(),
            }
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_path_when_it_is_absolute() {
            let path = Path::new("/home/user/file");
            assert_eq!(path, reader().to_absolute(path));
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_working_dir_prepended_when_path_is_relative() {
            let path = Path::new("./file");
            assert_eq!(
                Path::new("/usr/local/specdown/file"),
                reader().to_absolute(path)
            );
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_working_dir_prepended_when_path_contains_parent() {
            let path = Path::new("../file");
            assert_eq!(
                Path::new("/usr/local/specdown/../file"),
                reader().to_absolute(path)
            );
        }
    }
}
