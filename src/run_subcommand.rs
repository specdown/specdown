use clap::{Arg, SubCommand};
use std::fs;
use std::path::{Path, PathBuf};

use crate::parser;
use crate::results::basic_printer::BasicPrinter;
use crate::results::printer::Printer;
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
    let spec_files: Vec<&Path> = run_matches
        .values_of("spec-files")
        .expect("spec-files should always exist")
        .map(Path::new)
        .collect();

    let running_dir = run_matches.value_of("running-dir").map(Path::new);
    let shell_cmd = run_matches.value_of("shell-command").unwrap();

    execute_run(&spec_files, shell_cmd, running_dir);
}

fn execute_run(spec_files: &[&Path], shell_cmd: &str, running_dir: Option<&Path>) {
    let printer: Box<dyn Printer> = Box::new(BasicPrinter::new());
    let spec_dir = std::env::current_dir().expect("Failed to get current working directory");

    if let Some(dir) = running_dir {
        fs::create_dir_all(dir).expect("Failed to create running directory");
        std::env::set_current_dir(dir).expect("Failed to set running directory");
    }

    for spec_file in spec_files {
        printer.print_spec_file(spec_file);
        let contents = fs::read_to_string(to_absolute(spec_file, &spec_dir))
            .expect("failed to read spec file");
        let actions = parser::parse(&contents);

        match actions {
            Ok(action_list) => run_actions(&action_list, shell_cmd, &*printer),
            Err(err) => {
                (*printer).print_error(&Error::RunFailed {
                    message: err.to_string(),
                });
                std::process::exit(1)
            }
        }
    }
}

fn to_absolute(path: &Path, working_dir: &Path) -> PathBuf {
    if path.has_root() {
        path.to_path_buf()
    } else {
        working_dir.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::to_absolute;

    mod to_absolute {
        use super::to_absolute;
        use std::path::Path;

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_path_when_it_is_absolute() {
            let path = Path::new("/home/user/file");
            let working_dir = Path::new("/var");
            assert_eq!(path, to_absolute(path, working_dir));
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_working_dir_prepended_when_path_is_relative() {
            let path = Path::new("./file");
            let working_dir = Path::new("/var");
            assert_eq!(Path::new("/var/file"), to_absolute(path, working_dir));
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_working_dir_prepended_when_path_contains_parent() {
            let path = Path::new("../file");
            let working_dir = Path::new("/var/lib");
            assert_eq!(
                Path::new("/var/lib/../file"),
                to_absolute(path, working_dir)
            );
        }
    }
}
