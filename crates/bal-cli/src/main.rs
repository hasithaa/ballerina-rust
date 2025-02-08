use clap::Parser;

#[derive(Parser)]
#[command(name = "ballerina-rust")]
#[command(author = "Hasitha Aravinda <mail.hasitha27@gmail.com>")]
#[command(version = "0.1.0")]
#[command(about = "Ballerina compiler implemented in Rust", long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,
    
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    println!("Processing file: {}", cli.input);
}

mod commands;
mod config; 