extern crate clap;

use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let spec_file = Arg::with_name("spec-file")
        .index(1)
        .help("The spec file to run")
        .required(true);

    let output_file = Arg::with_name("output-file")
        .long("output-file")
        .short("o")
        .takes_value(true)
        .help("Location of where to save the generated MarkDown")
        .required(false);

    let run = SubCommand::with_name("run")
        .about("Runs a given Markdown Specification.")
        .arg(spec_file)
        .arg(output_file);

    let app = App::new("specdown")
        .about("A tool to test markdown files and drive devlopment from documentation.")
        .subcommand(run)
        .setting(AppSettings::ArgRequiredElseHelp);

    let _matches = app.get_matches();

    // let name = matches.value_of("name")
    //     .expect("This can't be None, we said it was required");

    // println!("Hello, {}!", name);
}
