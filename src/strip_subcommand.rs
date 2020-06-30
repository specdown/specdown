use crate::parser;
use clap::{Arg, SubCommand};
use std::fs;
use std::path::Path;

pub const NAME: &str = "strip";

pub fn create() -> clap::App<'static, 'static> {
    let spec_file = Arg::with_name("spec-file")
        .index(1)
        .help("The spec file to strip specdown functions from")
        .required(true);

    SubCommand::with_name(NAME)
        .about("Outputs a version of the markdown with all specdown functions removed")
        .arg(spec_file)
}

pub fn execute(run_matches: &clap::ArgMatches<'_>) {
    let spec_file = run_matches
        .value_of("spec-file")
        .map(Path::new)
        .expect("spec-file should always exist");

    execute_strip(spec_file);
}

fn execute_strip(spec_file: &Path) {
    let contents = fs::read_to_string(spec_file).expect("failed to read spec file");
    let stripped = parser::strip(&contents).expect("stripping to work");
    println!("{}", stripped)
}
