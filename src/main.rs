use envsetup::run;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Here,
    Get { repo: String },
}

fn main() {
    println!("envsetup v{}", env!("CARGO_PKG_VERSION"));
    // todo: move this into lib.rs
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Get { repo }) => println!("Repo: {}", repo),
        Some(Commands::Here) => run("envsetup.yml"),
        None => {
            println!("No command specified");
        }
    }
}
