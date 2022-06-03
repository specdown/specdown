use crate::parsers;
use clap::Args;
use std::fs;
use std::path::Path;

#[derive(Args)]
pub struct Arguments {
    /// The spec file to strip specdown functions from
    spec_file: String,
}

pub fn execute(args: &Arguments) {
    let spec_file = Path::new(&args.spec_file);
    let contents = fs::read_to_string(spec_file).expect("failed to read spec file");
    let stripped = parsers::strip(&contents);
    println!("{}", stripped);
}
