extern crate clap;

extern crate nom;

use clap::{App, AppSettings};

mod commands;
mod parser;
mod results;
mod runner;
mod types;

fn main() {
    let app = App::new("specdown")
        .about("A tool to test markdown files and drive devlopment from documentation.")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(commands::run::create())
        .subcommand(commands::strip::create())
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    if matches.is_present(commands::run::NAME) {
        let run_matches = matches.subcommand_matches(commands::run::NAME).unwrap();
        commands::run::execute(run_matches);
    } else if matches.is_present(commands::strip::NAME) {
        let strip_matches = matches.subcommand_matches(commands::strip::NAME).unwrap();
        commands::strip::execute(strip_matches);
    }
}
