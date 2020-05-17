use clap::{Arg, SubCommand};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::parser;
use crate::runner::state::State;
use crate::runner::test_result::TestResult;
use crate::types::{Action, ScriptCode, ScriptName, Source, VerifyValue};

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

fn run_actions(actions: &[Action]) {
    let mut state = State::new();
    println!("Found {} actions", actions.len());

    for action in actions {
        run_action(action, &mut state);
    }

    println!(
        "Ran {} actions",
        state.number_of_scripts() + state.number_of_verifies()
    );

    if !state.is_success() {
        std::process::exit(1);
    }
}

fn run_action(action: &Action, state: &mut State) {
    let action_number = state.number_of_scripts() + state.number_of_verifies() + 1;
    println!("## Running action {}\n", action_number);

    match action {
        Action::Script(name, code) => run_script(name, code, state),
        Action::Verify(source, value) => run_verify(source, value, state),
    }
}

fn run_script(name: &ScriptName, code: &ScriptCode, state: &mut State) {
    let ScriptName(name_string) = name;
    let ScriptCode(code_string) = code;

    println!("### Running script {}\n", name_string);

    let result = Command::new("sh").arg("-c").arg(code_string).output();

    match result {
        Ok(output) => {
            let output_string = String::from_utf8_lossy(&output.stdout).to_string();
            let result = TestResult::ScriptResult {
                name: name_string.to_string(),
                exit_code: 0,
                script: code_string.to_string(),
                output: output_string.clone(),
                stdout: "FIXME stderr".to_string(),
                stderr: "FIXME stderr1".to_string(),
                success: true,
            };
            state.add_result(&result);
            println!("**Result**: success\n");
        }
        Err(_err) => println!("**Result**: failed"),
    }
}

fn run_verify(source: &Source, value: &VerifyValue, state: &mut State) {
    let Source {
        name: ScriptName(script_name),
        stream: _stream,
    } = source;
    let VerifyValue(value_string) = value;

    println!("Running verify against output from {}\n", script_name);
    let got = state.get_script_output(script_name).expect("failed");

    let result = TestResult::VerifyResult {
        script_name: script_name.to_string(),
        stream: "FIXME output".to_string(),
        expected: value_string.to_string(),
        got: got.to_string(),
        success: value_string == got,
    };

    if value_string == got {
        println!("Result: success\n");
    } else {
        println!("Result: failed\n");
        println!("#### Expected\n");
        println!("```\n{}\n```\n", value_string);
        println!("#### Got\n");
        println!("```\n{}\n```\n", got);
    }

    state.add_result(&result);
}
