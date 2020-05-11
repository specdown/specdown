extern crate clap;

use clap::{App, AppSettings};

mod run_subcommand;
mod parser;
mod types;

fn main() {
    let app = App::new("specdown")
        .about("A tool to test markdown files and drive devlopment from documentation.")
        .subcommand(run_subcommand::create())
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    if matches.is_present("run") {
        let run_matches = matches.subcommand_matches("run").unwrap();
        run_subcommand::execute(run_matches);
    }
}
