use envsetup::run;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// Whether to automatically install all the necessary components on your system
    /// if they are not present
    #[clap(short, long)]
    auto_install: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs envsetup in this folder. Looks for envsetup.yml to perform the setup
    /// tasks
    Here,
    /// Pulls a repo from GitHub and automatically runs envsetup in the project
    Get {
        /// The repo to pull. Should be in the format username/repo (e.g. rust-lang/rust)
        repo: String,
    },
}

fn main() {
    //println!("envsetup v{}", env!("CARGO_PKG_VERSION"));
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
