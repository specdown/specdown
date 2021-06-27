use clap::{Arg, SubCommand};
use std::fs;
use std::path::{Path, PathBuf};

use crate::parser;
use crate::results::basic_printer::BasicPrinter;
use crate::results::printer::{PrintItem, Printer};
use crate::runner::{run_actions, Error};

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
    let shell_cmd = run_matches.value_of("shell-command").unwrap();
    let spec_dir = std::env::current_dir().expect("Failed to get current working directory");
    let printer = Box::new(BasicPrinter::new());

    let command = RunCommand {
        spec_files,
        spec_dir,
        shell_cmd: shell_cmd.to_string(),
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
    pub fn execute(&self) {
        self.change_to_running_directory();

        for spec_file in &self.spec_files {
            self.run_spec_file(&spec_file);
        }
    }

    fn run_spec_file(&self, spec_file: &Path) {
        self.printer
            .print(&PrintItem::SpecFileName(spec_file.to_path_buf()));

        let contents =
            fs::read_to_string(self.to_absolute(spec_file)).expect("failed to read spec file");
        let actions = parser::parse(&contents);

        match actions {
            Ok(action_list) => run_actions(&action_list, &self.shell_cmd, &*self.printer),
            Err(err) => {
                let error = Error::RunFailed {
                    message: err.to_string(),
                };
                self.printer.print(&PrintItem::RunError(error));
                std::process::exit(1)
            }
        }
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
