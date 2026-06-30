use std::path::PathBuf;

pub use arguments::Arguments;
use arguments::ExecutorKind;
use file_reader::FileReader;
use run_command::RunCommand;

use crate::config::Config;
use crate::exit_codes::ExitCode;
use crate::results::basic_printer::BasicPrinter;
use crate::results::Printer;
use crate::runner::shell_executor::ShellExecutor;
use crate::runner::{Error, RunEvent};
use crate::workspace::{ExistingDir, TemporaryDirectory, Workspace};

mod arguments;
mod exit_code;
mod file_reader;
mod run_command;

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
    let temp_workspace_dir = args.temporary_workspace_dir;
    let workspace_init_command = args.workspace_init_command.clone();
    let shell_cmd = args.shell_command.clone();
    let mut env = parse_environment_variables(&args.env);

    let unset_env = args.unset_env.clone();
    let paths = args.add_path.clone();
    let current_dir = std::env::current_dir().expect("Failed to get current workspace directory");
    let file_reader = FileReader::new(current_dir.clone());

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

    match args.executor_config.executor {
        ExecutorKind::Shell => {
            ShellExecutor::new(&shell_cmd, &env, &unset_env, &paths).map(|e| RunCommand {
                spec_files: args.spec_files.clone(),
                executor: Box::new(e),
                working_dir: actual_working_dir,
                workspace_init_command,
                file_reader,
            })
        }
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
                )
                .map(|e| RunCommand {
                    spec_files: args.spec_files.clone(),
                    executor: Box::new(e),
                    working_dir: actual_working_dir,
                    workspace_init_command,
                    file_reader,
                })
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
