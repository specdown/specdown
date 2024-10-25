use clap::Args;
use std::path::PathBuf;

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
}
