use clap::{Arg, SubCommand};
use std::fs;
use std::path::Path;

use crate::parser;
use crate::types::Action;

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
    let spec_file = run_matches.value_of("spec-file").unwrap();
    execute_run(Path::new(spec_file));
}

fn execute_run(spec_file: &Path) {
    let contents = fs::read_to_string(spec_file).expect("failed to read spec file");
    let actions = parser::parse(&contents);

    match actions {
        Ok(a) => run_actions(&a),
        Err(err) => println!("{}", err),
    }
}

struct State {
    count: u32,
}

fn run_actions(actions: &[Action]) {
    let mut state = State { count: 0 };
    println!("Found {} actions", actions.len());

    for action in actions {
        run_action(action, &mut state);
    }

    println!("Ran {} actions", state.count);
}

fn run_action(action: &Action, state: &mut State) {
    match action {
        Action::Script(_name, _code) => println!("Running script"),
        Action::Verify(_source, _value) => println!("Running verify"),
    }

    state.count += 1;
}
