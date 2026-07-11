//! A tool to test markdown files and drive development from documentation

#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

use std::path::PathBuf;

use crate::config::Config;
use clap::{CommandFactory, Parser, Subcommand};

mod ansi;
mod commands;
mod config;
mod exit_codes;
mod parsers;
mod results;
mod runner;
mod types;
mod workspace;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Disables coloured output
    #[clap(long)]
    no_colour: bool,

    /// Load settings from a specific config file instead of looking for
    /// `specdown.toml` in the current directory
    #[clap(long, value_name = "PATH")]
    config: Option<PathBuf>,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Output completion for a shell of your choice
    Completion(commands::completion::Arguments),

    /// Runs a given Markdown Specification
    Run(Box<commands::run::RunSettings>),

    /// Outputs a version of the markdown with all specdown functions removed
    Strip(commands::strip::Arguments),
}

fn main() {
    let cli = Cli::parse();

    let config = Config {
        colour: !cli.no_colour,
        config_path: cli.config,
    };

    match cli.command {
        Commands::Completion(args) => {
            commands::completion::execute(&mut Cli::command(), &args);
        }
        Commands::Run(args) => {
            commands::run::execute(&config, &args);
        }
        Commands::Strip(args) => {
            commands::strip::execute(&args);
        }
    }
}
