mod cli_utils;
mod git;
mod toolchains;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

use crate::cli_utils::{execute_cmd_list};
use crate::git::{process_git_cmds, validate_git_conf};
use crate::toolchains::rust::ensure_rustup_present;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvSetupConfig {
    language: String,
    git: HashMap<String, String>,
    setup_cmds: Vec<String>,
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
            // todo: support for python toolchain validation
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

/// Processes the configuration object and runs appropriate checks and commands
fn process_config(conf: &EnvSetupConfig) {
    let language = &conf.language;
    let git_conf = &conf.git;
    let setup_cmds = &conf.setup_cmds;

    if !validate_language(language) {
        println!("Invalid language: {}", language);
        return;
    }

    if !validate_git_conf(&conf.git) {
        println!("Git configuration is invalid");
        return;
    }

    if setup_cmds.is_empty() {
        println!("No setup commands found");
        return;
    }

    let git_cmd_results = execute_cmd_list(&process_git_cmds(git_conf));
}




