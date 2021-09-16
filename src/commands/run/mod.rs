use std::path::Path;

use clap::{Arg, SubCommand};

use file_reader::FileReader;
use run_command::RunCommand;

use crate::config::Config;
use crate::exit_codes::ExitCode;
use crate::results::basic_printer::BasicPrinter;
use crate::results::Printer;
use crate::runner::Error;
use crate::runner::RunEvent;
use crate::runner::Shell;

mod file_reader;
mod run_command;

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

pub fn execute(config: &Config, run_matches: &clap::ArgMatches<'_>) {
    let events = create_run_command(run_matches).map_or_else(
        |err| vec![RunEvent::ErrorOccurred(err)],
        |command| command.execute(),
    );

    let mut printer = Box::new(BasicPrinter::new(config.colour));
    for event in &events {
        printer.print(event);
    }

    let exit_code = events_to_exit_code(&events);

    std::process::exit(exit_code as i32)
}

fn create_run_command(run_matches: &clap::ArgMatches<'_>) -> Result<RunCommand, Error> {
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
    let file_reader = FileReader::new(spec_dir);

    Shell::new(&shell_cmd).map(|executor| RunCommand {
        spec_files,
        executor: Box::new(executor),
        running_dir,
        file_reader,
    })
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
