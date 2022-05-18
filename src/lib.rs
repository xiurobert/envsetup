mod cli;
mod toolchains;
mod tools;
mod validations;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::cli::cli_utils::{ensure_tool_present, execute_cmd_list, execute_terminal_command};
use crate::toolchains::docker::ensure_docker_present;
use crate::toolchains::python::ensure_python_present;
use crate::toolchains::rust::ensure_rustup_present;
use crate::tools::git::{check_if_in_repo, process_git_cmds};
use crate::validations::validate_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvSetupConfig {
    language: String,
    git: HashMap<String, String>,
    setup_cmds: Option<Vec<String>>,
    container_system: Option<String>,
}

enum Language {
    Rust,
    Python,
}

/// Ingests a configuration file and returns a `EnvSetupConfig` struct
/// The configuration file should be a YAML file. This function will throw an error if
/// the file could not be deserialized into the EnvSetupConfig struct
pub fn ingest_configuration_file(config_path: &str) -> Result<EnvSetupConfig, Box<dyn Error>> {
    let conf_str = fs::read_to_string(config_path)?;
    let result: EnvSetupConfig = serde_yaml::from_str(&conf_str)?;
    Ok(result)
}

/// The main run function. Called from the main.rs file.
pub fn run(conf_path: &str) {
    let e_config = ingest_configuration_file(conf_path).unwrap_or_else(|e| {
        panic!("Error: {}", e);
    });
    process_config(&e_config);
}

/// Handles the "initial" setup for the environment.
fn process_language(language: &str) -> bool {
    match language {
        "rust" => {
            println!("exec: cargo build");
            execute_terminal_command("cargo build")
        }
        "python" => true,
        _ => false,
    }
}

/// Processes the configuration object and runs appropriate checks and commands
fn process_config(conf: &EnvSetupConfig) {
    if !validate_config(conf) {
        return;
    }
    if !check_if_in_repo(".") {
        println!("Executing git commands...");
        let _git_cmd_results = execute_cmd_list(&process_git_cmds(&conf.git));
        println!("Completed executing git commands!");
    } else {
        println!("Already in a git repository!");
    }
    if let Some(setup_cmds) = &conf.setup_cmds {
        println!("Running setup commands...");
        let _setup_results = execute_cmd_list(setup_cmds);
        println!("Completed setup commands!");
    }
}
