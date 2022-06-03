use std::path::{Path, PathBuf};

use clap::Args;

use file_reader::FileReader;
use run_command::RunCommand;

use crate::config::Config;
use crate::exit_codes::ExitCode;
use crate::results::basic_printer::BasicPrinter;
use crate::results::Printer;
use crate::runner::shell_executor::ShellExecutor;
use crate::runner::{Error, RunEvent};

mod exit_code;
mod file_reader;
mod run_command;

#[derive(Args)]
pub struct Arguments {
    /// The spec files to run
    spec_files: Vec<String>,

    /// Set the workspace directory
    #[clap(long)]
    workspace_dir: Option<String>,

    /// Create a temporary workspace directory
    #[clap(long)]
    temporary_workspace_dir: bool,

    /// The directory where commands will be executed. This is relative to the workspace dir
    #[clap(long)]
    working_dir: Option<String>,

    /// A command to run in the workspace before running the specs
    #[clap(long)]
    workspace_init_command: Option<String>,

    /// The shell command used to execute script blocks
    #[clap(long, default_value_t = String::from("bash -c"))]
    shell_command: String,

    /// Set an environment variable (format: 'VAR_NAME=value')
    // todo: Add validator
    #[clap(long)]
    env: Vec<String>,

    /// Unset an environment variable
    #[clap(long)]
    unset_env: Vec<String>,

    /// Adds the given directory to PATH
    #[clap(long)]
    add_path: Vec<String>,
}

pub fn execute(config: &Config, args: &Arguments) {
    let events = create_run_command(args).map_or_else(
        |err| vec![RunEvent::ErrorOccurred(err)],
        |command| command.execute(),
    );

    let mut printer = BasicPrinter::new(config.colour);
    for event in &events {
        printer.print(event);
    }

    let exit_code = exit_code::from_events(&events);

    std::process::exit(exit_code as i32)
}

fn create_run_command(args: &Arguments) -> Result<RunCommand, Error> {
    let spec_files = args
        .spec_files
        .iter()
        .map(Path::new)
        .map(std::path::Path::to_path_buf)
        .collect();
    let specified_workspace_dir = args
        .workspace_dir
        .as_ref()
        .map(Path::new)
        .map(std::path::Path::to_path_buf);
    let temp_workspace_dir = args.temporary_workspace_dir;
    let working_dir = args
        .working_dir
        .as_ref()
        .map(Path::new)
        .map(std::path::Path::to_path_buf);
    let workspace_init_command = args.workspace_init_command.clone();
    let shell_cmd = args.shell_command.clone();
    let mut env = parse_environment_variables(&args.env);

    let unset_env = args.unset_env.clone();
    let paths = args.add_path.clone();
    let current_dir = std::env::current_dir().expect("Failed to get current workspace directory");
    let file_reader = FileReader::new(current_dir.clone());

    let workspace_dir = get_workspace_dir(specified_workspace_dir, temp_workspace_dir)
        .unwrap_or_else(|| current_dir.clone());

    std::fs::create_dir_all(&workspace_dir).expect("Failed to create workspace directory");
    let workspace_dir_canonicalized = std::fs::canonicalize(&workspace_dir)
        .unwrap_or_else(|_| panic!("Failed to canonicalize {:?}", workspace_dir));

    let actual_working_dir = working_dir.map_or_else(
        || workspace_dir_canonicalized.clone(),
        |dir| workspace_dir_canonicalized.clone().join(dir),
    );

    env.push((
        "SPECDOWN_START_DIR".to_string(),
        current_dir
            .into_os_string()
            .into_string()
            .expect("failed to convert start dir dir into a string"),
    ));

    env.push((
        "SPECDOWN_WORKSPACE_DIR".to_string(),
        workspace_dir_canonicalized
            .into_os_string()
            .into_string()
            .expect("failed to convert working dir into a string"),
    ));

    env.push((
        "SPECDOWN_WORKING_DIR".to_string(),
        actual_working_dir
            .clone()
            .into_os_string()
            .into_string()
            .expect("failed to convert working dir into a string"),
    ));

    let new_command = |e| RunCommand {
        spec_files,
        executor: Box::new(e),
        working_dir: actual_working_dir,
        workspace_init_command,
        file_reader,
    };

    ShellExecutor::new(&shell_cmd, &env, &unset_env, &paths).map(new_command)
}

fn get_workspace_dir(
    specified_workspace_dir: Option<PathBuf>,
    temp_workspace_dir: bool,
) -> Option<PathBuf> {
    if specified_workspace_dir.is_some() && temp_workspace_dir {
        println!(
            "  \u{2717} --workspace-dir and --temporary-workspace-dir cannot be specified at the same time"
        );
        std::process::exit(ExitCode::ErrorOccurred as i32)
    }

    if temp_workspace_dir {
        Some(
            tempfile::tempdir()
                .expect("Failed to create temporary workspace directory")
                .path()
                .to_path_buf(),
        )
    } else {
        specified_workspace_dir
    }
}

fn parse_environment_variables(strings: &[String]) -> Vec<(String, String)> {
    strings
        .iter()
        .map(|s| parse_environment_variable(s))
        .collect()
}

fn parse_environment_variable(string: &str) -> (String, String) {
    match string.splitn(2, '=').collect::<Vec<_>>()[..] {
        [] => panic!("Empty environment variable split"),
        [name] => (name.to_string(), "".to_string()),
        [name, value, ..] => (name.to_string(), value.to_string()),
    }
}
