mod cli_utils;
mod git;
mod toolchains;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::cli_utils::{ensure_tool_present, execute_cmd_list, execute_terminal_command};
use crate::git::{check_if_in_repo, process_git_cmds, validate_git_conf};
use crate::toolchains::python::ensure_python_present;
use crate::toolchains::rust::ensure_rustup_present;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvSetupConfig {
    language: String,
    git: HashMap<String, String>,
    setup_cmds: Option<Vec<String>>,
    container_system: Option<String>,
}

enum Language {
    Rust,
    Python
}

/// Ingests a configuration file and returns a `EnvSetupConfig` struct
/// The configuration file should be a YAML file. This function will throw an error if
/// the file could not be deserialized into the EnvSetupConfig struct
pub fn ingest_configuration_file(config_path: &str) -> Result<EnvSetupConfig, Box<dyn Error>> {
    let conf_str = fs::read_to_string(config_path)?;
    let result: EnvSetupConfig = serde_yaml::from_str(&conf_str)?;
    Ok(result)
}

/// Validates the language parameter in the configuration file and ensures that it is supported
/// by this tool
fn validate_language(language: &str) -> bool {
    match language {
        "rust" => {
            if ensure_rustup_present() {
                println!("Found rustup!");
                return true;
            }
            println!("Rustup not found!");
            false
        }
        "python" => {
            if ensure_python_present() {
                println!("Found python3!");
                return true;
            }
            println!("Could not find python3!");
            false
        }
        _ => false,
    }
}

/// The main run function. Called from the main.rs file.
pub fn run(conf_path: &str) {
    let e_config = ingest_configuration_file(conf_path).unwrap_or_else(|e| {
        panic!("Error: {}", e);
    });
    process_config(&e_config);
}

fn ensure_docker_present() -> bool {
    ensure_tool_present("docker")
}

fn validate_container_system(container_system: &str) -> bool {
    if container_system.is_empty() {
        return false;
    }
    match container_system {
        "docker" => {
            if !ensure_docker_present() {
                println!("Could not find docker on your system!");
                return false;
            }
            println!("Found docker!");
            true
        }
        _ => false,
    }
}

fn process_language(language: &str) -> bool {
    match language {
        "rust" => {
            println!("exec: cargo build");
            execute_terminal_command("cargo build")
        }
        "python" => {
            true
        }
        _ => false,
    }

}

/// Performs validation on the EnvSetupConfig struct
/// Returns a boolean based on the validity of the configuration
fn validate_config(conf: &EnvSetupConfig) -> bool {
    let language = &conf.language;
    let git_conf = &conf.git;
    let setup_cmds = &conf.setup_cmds;
    let container_system = &conf.container_system;

    if !validate_language(language) {
        println!("Invalid language: {}", language);
        return false;
    }

    if !process_language(language) {
        println!("Could not process language: {}", language);
        return false;
    }
    println!("Processed default commands for language: {}", language);

    if !validate_git_conf(git_conf) {
        println!("Git configuration is invalid");
        return false;
    }

    if let Some(container_system) = container_system {
        if !validate_container_system(container_system) {
            println!("Container system is not supported");
            return false;
        }
    }
    true
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
