extern crate clap;

extern crate nom;

use crate::config::Config;
use clap::{App, AppSettings, Arg};

mod ansi;
mod commands;
mod config;
mod exit_codes;
mod parser;
mod results;
mod runner;
mod types;

fn main() {
    let no_colour = Arg::with_name("no-colour")
        .long("no-colour")
        .takes_value(false)
        .help("Disables coloured output");

    let app = App::new("specdown")
        .about("A tool to test markdown files and drive devlopment from documentation.")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(no_colour)
        .subcommand(commands::run::create())
        .subcommand(commands::strip::create())
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    let config = Config {
        colour: !matches.is_present("no-colour"),
    };

    if matches.is_present(commands::run::NAME) {
        let run_matches = matches.subcommand_matches(commands::run::NAME).unwrap();
        commands::run::execute(&config, run_matches);
    } else if matches.is_present(commands::strip::NAME) {
        let strip_matches = matches.subcommand_matches(commands::strip::NAME).unwrap();
        commands::strip::execute(strip_matches);
    }
}
