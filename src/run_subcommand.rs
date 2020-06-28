use clap::{Arg, SubCommand};
use std::fs;
use std::path::Path;

use crate::parser;
use crate::results::basic_printer::BasicPrinter;
use crate::results::printer::Printer;
use crate::runner::run_actions;

pub const NAME: &str = "run";

pub fn create() -> clap::App<'static, 'static> {
    let spec_file = Arg::with_name("spec-file")
        .index(1)
        .help("The spec file to run")
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
    let spec_file = run_matches
        .value_of("spec-file")
        .map(Path::new)
        .expect("spec-file should always exist");

    let running_dir = run_matches.value_of("running-dir").map(Path::new);
    let shell_cmd = run_matches.value_of("shell-command").unwrap();

    execute_run(spec_file, shell_cmd, running_dir);
}

fn execute_run(spec_file: &Path, shell_cmd: &str, running_dir: Option<&Path>) {
    let printer: Box<dyn Printer> = Box::new(BasicPrinter::new());
    let contents = fs::read_to_string(spec_file).expect("failed to read spec file");
    let actions = parser::parse(&contents);

    if let Some(dir) = running_dir {
        fs::create_dir_all(dir).expect("Failed to create running directory");
        std::env::set_current_dir(dir).expect("Failed to set running directory");
    }

    match actions {
        Ok(action_list) => run_actions(&action_list, shell_cmd, &*printer),
        Err(err) => {
            println!("{}", err);
            std::process::exit(1)
        }
    }
}
