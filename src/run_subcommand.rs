use clap::{Arg, SubCommand};
use std::fs;
use std::path::Path;

use crate::parser;
use crate::results::basic_printer::BasicPrinter;
use crate::results::printer::Printer;
use crate::runner::run_actions;

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

    SubCommand::with_name("run")
        .about("Runs a given Markdown Specification.")
        .arg(spec_file)
        .arg(test_dir)
}

pub fn execute(run_matches: &clap::ArgMatches<'_>) {
    let spec_file = run_matches
        .value_of("spec-file")
        .map(Path::new)
        .expect("spec-file should always exist");

    let running_dir = run_matches.value_of("running-dir").map(Path::new);

    execute_run(spec_file, running_dir);
}

fn execute_run(spec_file: &Path, running_dir: Option<&Path>) {
    let printer: Box<dyn Printer> = Box::new(BasicPrinter::new());
    let contents = fs::read_to_string(spec_file).expect("failed to read spec file");
    let actions = parser::parse(&contents);

    if let Some(dir) = running_dir {
        std::env::set_current_dir(dir).expect("Failed to set running directory");
    }

    match actions {
        Ok(a) => run_actions(&a, &*printer),
        Err(err) => println!("{}", err),
    }
}
