use std::path::PathBuf;

use merge::Merge;

use file_reader::FileReader;
use run_command::RunCommand;
use settings::ExecutorKind;
pub use settings::RunSettings;

use crate::config::Config;
use crate::exit_codes::ExitCode;
use crate::results::basic_printer::BasicPrinter;
use crate::runner::shell_executor::ShellExecutor;
use crate::runner::{Error, RunEvent};
use crate::workspace::{ExistingDir, TemporaryDirectory, Workspace};

mod config_file;
mod exit_code;
mod file_discovery;
mod file_reader;
mod run_command;
mod settings;

/// The shell command used to invoke script blocks when neither the command
/// line nor `specdown.toml` sets one.
const DEFAULT_SHELL_COMMAND: &str = "bash -c";

/// The number of parallel jobs used when neither the command line nor
/// `specdown.toml` sets one.
const DEFAULT_JOBS: u32 = 1;

pub fn execute(config: &Config, args: &RunSettings) {
    let printer = BasicPrinter::new(config.colour);
    let printer_mutex =
        std::sync::Mutex::new(Box::new(printer) as Box<dyn crate::results::Printer>);

    let events = create_run_command(config, args).map_or_else(
        |err| {
            let events = vec![RunEvent::ErrorOccurred(err)];
            let mut guard = printer_mutex.lock().expect("printer mutex poisoned");
            for event in &events {
                guard.print(event);
            }
            events
        },
        |command| command.execute_with_printer(&printer_mutex),
    );

    let exit_code = exit_code::from_events(&events);

    std::process::exit(exit_code as i32)
}

fn create_run_command(config: &Config, cli_settings: &RunSettings) -> Result<RunCommand, Error> {
    let current_dir = std::env::current_dir().expect("Failed to get current workspace directory");

    let mut args = cli_settings.clone();
    let file_settings =
        config_file::load_run_settings(config.config_path.as_deref(), &current_dir)?;
    args.merge(file_settings);

    let temp_workspace_dir = args.temporary_workspace_dir;
    let workspace_init_command = args.workspace_init_command.clone();
    let shell_cmd = args
        .shell_command
        .clone()
        .unwrap_or_else(|| DEFAULT_SHELL_COMMAND.to_string());
    let mut env = parse_environment_variables(&args.env);

    // Resolve jobs: 0 means "run all in parallel" — map to CPU count.
    let jobs = match args.jobs.unwrap_or(DEFAULT_JOBS) {
        0 => num_cpus::get(),
        jobs => jobs as usize,
    };

    let unset_env = args.unset_env.clone();
    let paths = args.add_path.clone();
    let file_reader = FileReader::new(current_dir.clone());

    let spec_files =
        file_discovery::build_file_list(&args.spec_files, &current_dir, args.follow_links)?;

    let mut workspace = create_workspace(args.workspace_dir.clone(), temp_workspace_dir);
    workspace.initialize();

    let actual_working_dir = args.working_dir.clone().map_or_else(
        || workspace.dir().clone(),
        |dir| workspace.dir().clone().join(dir),
    );

    env.push((
        "SPECDOWN_START_DIR".to_string(),
        current_dir
            .into_os_string()
            .into_string()
            .expect("failed to convert start dir into a string"),
    ));

    env.push((
        "SPECDOWN_WORKSPACE_DIR".to_string(),
        workspace
            .dir()
            .clone()
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

    let new_command = |e: Box<dyn crate::runner::Executor>| RunCommand {
        spec_files: spec_files.clone(),
        executor: e,
        working_dir: actual_working_dir,
        workspace_init_command,
        file_reader,
        jobs,
    };

    match args.executor_config.executor.unwrap_or_default() {
        ExecutorKind::Shell => ShellExecutor::new(&shell_cmd, &env, &unset_env, &paths)
            .map(|e| new_command(Box::new(e))),
        ExecutorKind::Container => {
            #[cfg(feature = "container")]
            {
                let image = args
                    .executor_config
                    .container_image
                    .clone()
                    .unwrap_or_else(|| "bash:5".to_string());
                crate::runner::container_executor::ContainerExecutor::new::<String>(
                    &image,
                    &shell_cmd,
                    &env,
                    &unset_env,
                    &paths,
                    &args.executor_config.container_volumes,
                    "main",
                )
                .map(|e| new_command(Box::new(e)))
            }
            #[cfg(not(feature = "container"))]
            {
                Err(Error::ContainerFeatureNotEnabled)
            }
        }
    }
}

fn create_workspace(
    specified_workspace_dir: Option<PathBuf>,
    temp_workspace_dir: bool,
) -> Box<dyn Workspace> {
    if specified_workspace_dir.is_some() && temp_workspace_dir {
        println!(
            "  \u{2717} --workspace-dir and --temporary-workspace-dir cannot be specified at the same time"
        );
        std::process::exit(ExitCode::ErrorOccurred as i32)
    }

    if temp_workspace_dir {
        Box::new(TemporaryDirectory::create())
    } else {
        Box::new(ExistingDir::create(specified_workspace_dir.unwrap_or_else(
            || std::env::current_dir().expect("Failed to get current workspace directory"),
        )))
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
        [name] => (name.to_string(), String::new()),
        [name, value, ..] => (name.to_string(), value.to_string()),
    }
}
