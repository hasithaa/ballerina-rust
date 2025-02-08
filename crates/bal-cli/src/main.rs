use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ballerina-rust")]
#[command(author = "Hasitha Aravinda <mail.hasitha27@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Ballerina compiler implemented in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a Ballerina file
    Build {
        /// Input .bal file
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Build { input } => {
            commands::build(&input)
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

mod commands;
mod config; 