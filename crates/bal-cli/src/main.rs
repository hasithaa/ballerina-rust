use crate::config::Config;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ballerina-rust")]
#[command(author = "Hasitha Aravinda <mail.hasitha27@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Ballerina compiler implemented in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable debug output
    #[arg(short, long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a Ballerina project or file
    Build {
        /// Optional path to a .bal file or project directory.
        /// If not provided, attempts to build project in current directory
        input: Option<PathBuf>,
    },
    /// Clean the target directory of a Ballerina project
    Clean {
        /// Optional path to project directory.
        /// If not provided, attempts to clean project in current directory
        path: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new(cli.debug);

    let result = match cli.command {
        Commands::Build { input } => commands::build(input, &config),
        Commands::Clean { path } => commands::clean(path, &config),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

mod commands;
mod config;
mod dependency;
