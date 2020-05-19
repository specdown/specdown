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

    SubCommand::with_name("run")
        .about("Runs a given Markdown Specification.")
        .arg(spec_file)
}

pub fn execute(run_matches: &clap::ArgMatches<'_>) {
    let spec_file = run_matches
        .value_of("spec-file")
        .expect("spec-file should always exist");
    execute_run(Path::new(spec_file));
}

fn execute_run(spec_file: &Path) {
    let printer: Box<dyn Printer> = Box::new(BasicPrinter::new());
    let contents = fs::read_to_string(spec_file).expect("failed to read spec file");
    let actions = parser::parse(&contents);

    match actions {
        Ok(a) => run_actions(&a, &*printer),
        Err(err) => println!("{}", err),
    }
}
