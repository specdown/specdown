extern crate clap;

extern crate nom;

use clap::{App, AppSettings};

mod parser;
mod results;
mod run_subcommand;
mod runner;
mod strip_subcommand;
mod types;

fn main() {
    let app = App::new("specdown")
        .about("A tool to test markdown files and drive devlopment from documentation.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(run_subcommand::create())
        .subcommand(strip_subcommand::create())
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    if matches.is_present(run_subcommand::NAME) {
        let run_matches = matches.subcommand_matches(run_subcommand::NAME).unwrap();
        run_subcommand::execute(run_matches);
    } else if matches.is_present(strip_subcommand::NAME) {
        let strip_matches = matches.subcommand_matches(strip_subcommand::NAME).unwrap();
        strip_subcommand::execute(strip_matches);
    }
}
