use std::path::PathBuf;

use merge::Merge;

#[cfg(feature = "container")]
use executor_factory::ContainerExecutorFactory;
use executor_factory::{ExecutorFactory, ShellExecutorFactory};
use file_reader::FileReader;
use run_command::{RunCommand, RunMode};
use settings::ExecutorKind;
pub use settings::RunSettings;

use crate::config::Config;
use crate::exit_codes::ExitCode;
use crate::results::basic_printer::BasicPrinter;
use crate::runner::{Error, RunEvent};
use crate::workspace::{ExistingDir, TemporaryDirectory, Workspace};

mod config_file;
mod executor_factory;
mod exit_code;
mod file_discovery;
mod file_reader;
mod run_command;
mod settings;
mod specdown_env;

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
    let env = parse_environment_variables(&args.env);

    // Resolve jobs: 0 means "run all in parallel" — map to CPU count.
    let jobs = match args.jobs.unwrap_or(DEFAULT_JOBS) {
        0 => num_cpus::get(),
        jobs => jobs as usize,
    };

    let unset_env = args.unset_env.clone();
    let paths = args.add_path.clone();
    let file_reader = FileReader::new(current_dir.clone());
    let workspace_per_spec = args.workspace_per_spec;

    let spec_files =
        file_discovery::build_file_list(&args.spec_files, &current_dir, args.follow_links)?;

    validate_workspace_dir_conflict(args.workspace_dir.as_ref(), temp_workspace_dir);

    if let Some(message) =
        workspace_per_spec_validation_error(workspace_per_spec, temp_workspace_dir)
    {
        println!("  \u{2717} {message}");
        std::process::exit(ExitCode::ErrorOccurred as i32)
    }

    let factory = build_executor_factory(&args, shell_cmd, env, unset_env, paths)?;

    if workspace_per_spec {
        return Ok(RunCommand {
            spec_files,
            run_mode: RunMode::PerSpecWorkspace {
                factory,
                start_dir: current_dir,
                working_dir_suffix: args.working_dir.clone(),
            },
            workspace_init_command,
            file_reader,
            jobs,
        });
    }

    let mut workspace = create_workspace(args.workspace_dir.clone(), temp_workspace_dir);
    workspace.initialize();

    let actual_working_dir = args.working_dir.clone().map_or_else(
        || workspace.dir().clone(),
        |dir| workspace.dir().clone().join(dir),
    );

    let extra_env = specdown_env::build(&current_dir, workspace.dir(), &actual_working_dir);
    let executor = factory.build("main", &extra_env, &actual_working_dir)?;

    Ok(RunCommand {
        spec_files,
        run_mode: RunMode::SharedWorkspace {
            executor,
            working_dir: actual_working_dir,
        },
        workspace_init_command,
        file_reader,
        jobs,
    })
}

/// Builds the `ExecutorFactory` matching the configured executor kind
/// (`--executor shell` or `--executor container`), shared by both the
/// single-shared-workspace path and the `--workspace-per-spec` path.
///
/// `Result` is only "unnecessary" when the `container` feature is enabled
/// (every arm becomes infallible); without it, the container arm returns
/// `Err(ContainerFeatureNotEnabled)`, so the wrapping is genuinely needed
/// depending on build configuration.
#[allow(clippy::unnecessary_wraps)]
fn build_executor_factory(
    args: &RunSettings,
    shell_cmd: String,
    env: Vec<(String, String)>,
    unset_env: Vec<String>,
    paths: Vec<String>,
) -> Result<Box<dyn ExecutorFactory>, Error> {
    match args.executor_config.executor.unwrap_or_default() {
        ExecutorKind::Shell => Ok(Box::new(ShellExecutorFactory {
            shell_cmd,
            base_env: env,
            unset_env,
            paths,
        })),
        ExecutorKind::Container => {
            #[cfg(feature = "container")]
            {
                let image = args
                    .executor_config
                    .container_image
                    .clone()
                    .unwrap_or_else(|| "bash:5".to_string());
                Ok(Box::new(ContainerExecutorFactory {
                    image,
                    shell_cmd,
                    base_env: env,
                    unset_env,
                    paths,
                    container_volumes: args.executor_config.container_volumes.clone(),
                }))
            }
            #[cfg(not(feature = "container"))]
            {
                Err(Error::ContainerFeatureNotEnabled)
            }
        }
    }
}

/// Prints and exits (matching the style of the existing
/// `--workspace-dir`/`--temporary-workspace-dir` conflict check) if
/// `--workspace-dir` and `--temporary-workspace-dir` are both set.
fn validate_workspace_dir_conflict(
    specified_workspace_dir: Option<&PathBuf>,
    temp_workspace_dir: bool,
) {
    if specified_workspace_dir.is_some() && temp_workspace_dir {
        println!(
            "  \u{2717} --workspace-dir and --temporary-workspace-dir cannot be specified at the same time"
        );
        std::process::exit(ExitCode::ErrorOccurred as i32)
    }
}

/// Returns the error message to print if `--workspace-per-spec` is set
/// without `--temporary-workspace-dir`, or `None` if the combination is valid.
fn workspace_per_spec_validation_error(
    workspace_per_spec: bool,
    temp_workspace_dir: bool,
) -> Option<&'static str> {
    if workspace_per_spec && !temp_workspace_dir {
        Some("--workspace-per-spec requires --temporary-workspace-dir")
    } else {
        None
    }
}

fn create_workspace(
    specified_workspace_dir: Option<PathBuf>,
    temp_workspace_dir: bool,
) -> Box<dyn Workspace> {
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

#[cfg(test)]
mod tests {
    use super::workspace_per_spec_validation_error;

    #[test]
    fn errors_when_workspace_per_spec_is_set_without_temporary_workspace_dir() {
        assert_eq!(
            workspace_per_spec_validation_error(true, false),
            Some("--workspace-per-spec requires --temporary-workspace-dir")
        );
    }

    #[test]
    fn is_valid_when_both_are_set() {
        assert_eq!(workspace_per_spec_validation_error(true, true), None);
    }

    #[test]
    fn is_valid_when_workspace_per_spec_is_not_set() {
        assert_eq!(workspace_per_spec_validation_error(false, false), None);
        assert_eq!(workspace_per_spec_validation_error(false, true), None);
    }
}
