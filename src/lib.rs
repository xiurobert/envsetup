mod cli;
mod config_file;
mod os;
mod toolchains;
mod tools;
mod validations;
pub mod git;
pub mod cli_utils;


use std::error::Error;
use std::fs;

use crate::config_file::{EnvSetupConfig, Language};
use crate::toolchains::docker::ensure_docker_present;
use crate::validations::validate_config;

/// Ingests a configuration file and returns a `EnvSetupConfig` struct
/// The configuration file should be a YAML file. This function will throw an error if
/// the file could not be deserialized into the EnvSetupConfig struct
fn ingest_configuration_file(config_path: &str) -> Result<EnvSetupConfig, Box<dyn Error>> {
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
fn process_language(language: &Language) -> bool {
    match language {
        Language::Rust => {
            println!("exec: cargo build");
            execute_terminal_command("cargo build").is_ok()
        }
        Language::Python => true
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
