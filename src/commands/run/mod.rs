use std::path::Path;

use clap::{Arg, SubCommand};

use file_reader::FileReader;
use run_command::RunCommand;

use crate::config::Config;
use crate::results::basic_printer::BasicPrinter;
use crate::results::Printer;
use crate::runner::shell_executor::ShellExecutor;
use crate::runner::{Error, RunEvent};

mod exit_code;
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

    let env = Arg::with_name("env")
        .long("env")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1)
        .help("Set an environment variable (format: 'VAR_NAME=value')")
        .required(false);

    let unset_env = Arg::with_name("unset-env")
        .long("unset-env")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1)
        .help("Unset an environment variable")
        .required(false);

    let add_path = Arg::with_name("add-path")
        .long("add-path")
        .takes_value(true)
        .multiple(true)
        .number_of_values(1)
        .help("Adds the given directory to PATH")
        .required(false);

    SubCommand::with_name(NAME)
        .about("Runs a given Markdown Specification")
        .arg(spec_file)
        .arg(test_dir)
        .arg(shell_cmd)
        .arg(env)
        .arg(unset_env)
        .arg(add_path)
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

    let exit_code = exit_code::from_events(&events);

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
    let env = run_matches
        .values_of("env")
        .map_or(vec![], parse_environment_variables);
    let unset_env = run_matches.values_of("unset-env").map_or(vec![], |v| {
        v.map(std::string::ToString::to_string).collect()
    });
    let paths = run_matches
        .values_of("add-path")
        .map_or(vec![], std::iter::Iterator::collect);
    let spec_dir = std::env::current_dir().expect("Failed to get current working directory");
    let file_reader = FileReader::new(spec_dir);

    let new_command = |e| RunCommand {
        spec_files,
        executor: Box::new(e),
        running_dir,
        file_reader,
    };

    ShellExecutor::new(&shell_cmd, &env, &unset_env, &paths).map(new_command)
}

fn parse_environment_variables<'a>(
    strings: impl Iterator<Item = &'a str>,
) -> Vec<(String, String)> {
    strings.map(parse_environment_variable).collect()
}

fn parse_environment_variable(string: &str) -> (String, String) {
    match string.splitn(2, '=').collect::<Vec<_>>()[..] {
        [] => panic!("Empty environment variable split"),
        [name] => (name.to_string(), "".to_string()),
        [name, value, ..] => (name.to_string(), value.to_string()),
    }
}
