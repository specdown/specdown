use clap::{Arg, SubCommand};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::parser;
use crate::results::test_result::TestResult;
use crate::runner::state::State;
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

    for action in actions {
        let result = run_action(action, &mut state);
        match result {
            Ok(result) => print_result(&result),
            Err(_err) => println!("Action failed..."),
        }
    }

    if !state.is_success() {
        std::process::exit(1);
    }
}

fn print_result(result: &TestResult) {
    match result {
        TestResult::ScriptResult { name, success, .. } => println!(
            "Script {} {}",
            name,
            if *success { "succeeded" } else { "failed" }
        ),
        TestResult::VerifyResult {
            script_name,
            success,
            ..
        } => println!(
            "Verify output from {} {}",
            script_name,
            if *success { "succeeded" } else { "failed" }
        ),
    }
}

fn run_action(action: &Action, state: &mut State) -> Result<TestResult, RunnerError> {
    match action {
        Action::Script(name, code) => run_script(name, code, state),
        Action::Verify(source, value) => run_verify(source, value, state),
    }
}

enum RunnerError {
    CommandFailed,
}

fn run_script(
    name: &ScriptName,
    code: &ScriptCode,
    state: &mut State,
) -> Result<TestResult, RunnerError> {
    let ScriptName(name_string) = name;
    let ScriptCode(code_string) = code;

    let command_result = Command::new("sh").arg("-c").arg(code_string).output();

    match command_result {
        Ok(output) => {
            let output_string = String::from_utf8_lossy(&output.stdout).to_string();
            let result = TestResult::ScriptResult {
                name: name_string.to_string(),
                exit_code: 0,
                script: code_string.to_string(),
                output: output_string,
                stdout: "FIXME stderr".to_string(),
                stderr: "FIXME stderr1".to_string(),
                success: true,
            };
            state.add_result(&result);
            Ok(result)
        }
        Err(_err) => Err(RunnerError::CommandFailed),
    }
}

fn run_verify(
    source: &Source,
    value: &VerifyValue,
    state: &mut State,
) -> Result<TestResult, RunnerError> {
    let Source {
        name: ScriptName(script_name),
        stream: _stream,
    } = source;
    let VerifyValue(value_string) = value;

    let got = state.get_script_output(script_name).expect("failed");

    let result = TestResult::VerifyResult {
        script_name: script_name.to_string(),
        stream: "FIXME output".to_string(),
        expected: value_string.to_string(),
        got: got.to_string(),
        success: value_string == got,
    };

    state.add_result(&result);

    Ok(result)
}
