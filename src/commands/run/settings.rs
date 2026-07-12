use clap::{Args, ValueEnum};
use merge::Merge;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Deserializes the `[run.env]` table (a `KEY = "value"` dictionary) into
/// the `"KEY=VALUE"` string form used internally and by the `--env` CLI flag.
///
/// A `BTreeMap` is used so the resulting order is deterministic (sorted by
/// key) regardless of the order keys appear in the TOML file.
fn deserialize_env_map<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let map = BTreeMap::<String, String>::deserialize(deserializer)?;
    Ok(map.into_iter().map(|(k, v)| format!("{k}={v}")).collect())
}

/// The executor backend to use for running script blocks.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, ValueEnum, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExecutorKind {
    /// Run scripts directly in the host shell (default).
    #[default]
    Shell,
    /// Run scripts inside a Docker container via the Docker socket API.
    Container,
}

/// The executor backend for running spec scripts.
///
/// Every field is optional so that command-line values, `specdown.toml`
/// values and hard-coded defaults can be layered together: `None`/empty
/// means "not specified by this source".
#[derive(Args, Deserialize, Merge, Debug, Clone, Eq, PartialEq, Default)]
#[serde(default, deny_unknown_fields)]
pub struct ExecutorConfig {
    /// The executor to use for running scripts.
    ///
    /// `shell` (default) runs scripts directly in the host shell.
    /// `container` runs scripts inside a Docker container via the Docker
    /// Engine socket API. Requires specdown to be built with the
    /// `container` feature.
    #[clap(long)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub executor: Option<ExecutorKind>,

    /// The Docker image to use when `--executor container` is selected.
    ///
    /// Ignored when the shell executor is used.
    #[clap(long)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub container_image: Option<String>,

    /// Mount a host directory into the container (repeatable).
    ///
    /// Uses Docker CLI bind-mount syntax: `<host_path>:<container_path>[:options]`.
    /// For example, `--container-volume /host/data:/data` mounts the host
    /// directory `/host/data` at `/data` inside the container. Append `:ro`
    /// for a read-only mount.
    ///
    /// Only effective with `--executor container`.
    #[clap(long = "container-volume", value_name = "HOST:CONTAINER[:OPTIONS]")]
    #[merge(strategy = merge::vec::overwrite_empty)]
    pub container_volumes: Vec<String>,
}

/// The settings that control how `specdown run` behaves.
///
/// This struct is the single source of truth for every run setting: it is
/// parsed directly from the command line (via `clap::Args`), parsed from the
/// `[run]` table of a `specdown.toml` file (via `serde::Deserialize`), and
/// merged between the two (via `merge::Merge`, command-line values winning).
/// Adding a new setting only requires adding one field here.
#[derive(Args, Deserialize, Merge, Debug, Clone, Default)]
#[serde(default, deny_unknown_fields)]
pub struct RunSettings {
    /// The spec files to run
    #[serde(rename = "files")]
    #[merge(strategy = merge::vec::overwrite_empty)]
    pub spec_files: Vec<PathBuf>,

    /// Set the workspace directory
    #[clap(long)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub workspace_dir: Option<PathBuf>,

    /// Create a temporary workspace directory
    #[clap(long)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub temporary_workspace_dir: bool,

    /// The directory where commands will be executed. This is relative to the workspace dir
    #[clap(long)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub working_dir: Option<PathBuf>,

    /// A command to run in the workspace before running the specs
    #[clap(long)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub workspace_init_command: Option<String>,

    /// The shell command used to execute script blocks
    ///
    /// Defaults to "bash -c" if not set via this flag or the `[run]` table
    /// in `specdown.toml`.
    #[clap(long)]
    #[merge(strategy = merge::option::overwrite_none)]
    pub shell_command: Option<String>,

    #[allow(clippy::doc_markdown)]
    /// Set an environment variable (format: 'VAR_NAME=value')
    // todo: Add validator
    #[clap(long)]
    #[serde(deserialize_with = "deserialize_env_map")]
    #[merge(strategy = merge::vec::overwrite_empty)]
    pub env: Vec<String>,

    /// Unset an environment variable
    #[clap(long)]
    #[merge(strategy = merge::vec::overwrite_empty)]
    pub unset_env: Vec<String>,

    /// Adds the given directory to PATH
    #[clap(long)]
    #[merge(strategy = merge::vec::overwrite_empty)]
    pub add_path: Vec<String>,

    /// Number of parallel jobs to run (0 = all CPUs, default = 1 for backward compatibility)
    #[clap(short = 'j', long = "jobs", allow_hyphen_values = true, value_parser = clap::value_parser!(u32).range(0..))]
    #[merge(strategy = merge::option::overwrite_none)]
    pub jobs: Option<u32>,

    /// Executor configuration (shell vs container)
    #[clap(flatten)]
    #[serde(rename = "executor")]
    pub executor_config: ExecutorConfig,

    /// Follow local Markdown links found in spec files and run every linked
    /// file too, recursively. Files are deduplicated by canonical path, so
    /// link cycles are handled safely and each file only runs once.
    ///
    /// This can also be enabled via a `specdown.toml` config file
    /// (`follow_links = true`) in the current directory; either the flag or
    /// the config file being set enables the behaviour.
    #[clap(long)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub follow_links: bool,

    /// Create a new temporary workspace directory for every spec file that
    /// is run, instead of sharing one temporary workspace across the whole
    /// invocation. `workspace_init_command` (if set) is re-run for each new
    /// per-spec workspace, before that spec file's actions run.
    ///
    /// Requires `--temporary-workspace-dir` (or `temporary_workspace_dir =
    /// true` in `specdown.toml`) to also be set; specdown errors otherwise.
    #[clap(long)]
    #[merge(strategy = merge::bool::overwrite_false)]
    pub workspace_per_spec: bool,
}
