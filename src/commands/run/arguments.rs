use clap::{Args, ValueEnum};
use std::path::PathBuf;

/// The executor backend to use for running script blocks.
#[derive(Debug, Clone, Eq, PartialEq, Default, ValueEnum)]
pub enum ExecutorKind {
    /// Run scripts directly in the host shell (default).
    #[default]
    Shell,
    /// Run scripts inside a Docker container via the Docker socket API.
    Container,
}

/// The executor backend for running spec scripts.
#[derive(Args, Debug, Clone, Eq, PartialEq, Default)]
pub struct ExecutorConfig {
    /// The executor to use for running scripts.
    ///
    /// `shell` (default) runs scripts directly in the host shell.
    /// `container` runs scripts inside a Docker container via the Docker
    /// Engine socket API. Requires specdown to be built with the
    /// `container` feature.
    #[clap(long, default_value = "shell")]
    pub executor: ExecutorKind,

    /// The Docker image to use when `--executor container` is selected.
    ///
    /// Ignored when the shell executor is used.
    #[clap(long)]
    pub container_image: Option<String>,
}

#[derive(Args)]
pub struct Arguments {
    /// The spec files to run
    pub spec_files: Vec<PathBuf>,

    /// Set the workspace directory
    #[clap(long)]
    pub workspace_dir: Option<PathBuf>,

    /// Create a temporary workspace directory
    #[clap(long)]
    pub temporary_workspace_dir: bool,

    /// The directory where commands will be executed. This is relative to the workspace dir
    #[clap(long)]
    pub working_dir: Option<PathBuf>,

    /// A command to run in the workspace before running the specs
    #[clap(long)]
    pub workspace_init_command: Option<String>,

    /// The shell command used to execute script blocks
    #[clap(long, default_value_t = String::from("bash -c"))]
    pub shell_command: String,

    #[allow(clippy::doc_markdown)]
    /// Set an environment variable (format: 'VAR_NAME=value')
    // todo: Add validator
    #[clap(long)]
    pub env: Vec<String>,

    /// Unset an environment variable
    #[clap(long)]
    pub unset_env: Vec<String>,

    /// Adds the given directory to PATH
    #[clap(long)]
    pub add_path: Vec<String>,

    /// Executor configuration (shell vs container)
    #[clap(flatten)]
    pub executor_config: ExecutorConfig,
}
