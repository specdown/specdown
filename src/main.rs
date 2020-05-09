extern crate clap;

use clap::{Arg, App};

fn main() {
    let app = App::new("specdown");

    let spec_file = Arg::with_name("spec-file")
        .long("spec-file")
        .takes_value(true)
        .help("The spec file to run")
        .required(true);

    let output_file = Arg::with_name("output-file")
        .long("output-file")
        .takes_value(true)
        .help("The generated output file")
        .required(true);

    let app = app.arg(spec_file);
    let app = app.arg(output_file);

    let _matches = app.get_matches();

    // let name = matches.value_of("name")
    //     .expect("This can't be None, we said it was required");

    // println!("Hello, {}!", name);
}
