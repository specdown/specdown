use crate::parsers;
use clap::Args;
use std::fs;
use std::path::PathBuf;

#[derive(Args)]
pub struct Arguments {
    /// The spec file to strip specdown functions from
    #[clap()]
    spec_file: PathBuf,
}

pub fn execute(args: &Arguments) {
    let contents = fs::read_to_string(&args.spec_file).expect("failed to read spec file");
    let stripped = parsers::strip(&contents);
    println!("{stripped}");
}
