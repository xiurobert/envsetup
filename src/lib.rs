use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process::Command;

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

/// Validates the git configuration in the configuration file and ensures that the git
/// configuration complies with the specification and is supported by this program
fn validate_git_conf(git_conf: &HashMap<String, String>) -> bool {
    if git_conf.is_empty() {
        return false;
    }
    let accepted_values = [String::from("repo"), String::from("branch")];
    let mut result = true;
    for key in git_conf.keys() {
        if !accepted_values.contains(key) {
            result = false;
        }
    }
    result
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

/// Execute a list of commands
fn execute_cmd_list(cmd_list: &[String]) -> Vec<bool> {
    let mut results = Vec::new();
    for cmd in cmd_list.iter() {
        results.push(execute_terminal_command(cmd));
    }
    results
}

/// Execute a command "in the terminal"
/// This function tries to emulate the experience of typing the command in your terminal
/// and pressing enter.
fn execute_terminal_command(cmd: &str) -> bool {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(cmd).output()
    } else {
        Command::new("sh").arg("-c").arg(cmd).output()
    };
    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}

/// Processes the git commands as defined in the envsetup.yml configuration file
/// And dumps them out as a list of console-executable commands
/// Silently discards any commands that are not supported
fn process_git_cmds(git_conf: &HashMap<String, String>) -> Vec<String> {
    let mut git_cmds = Vec::new();
    for (key, value) in git_conf {
        match key.as_ref() {
            "repo" => {
                let cmd = format!("git clone {}", value);
                git_cmds.push(cmd);
            }
            "branch" => {
                let cmd = format!("git checkout {}", value);
                git_cmds.push(cmd);
            }
            _ => {}
        }
    }
    git_cmds
}

/// Checks if rustup is installed
fn ensure_rustup_present() -> bool {
    if cfg!(target_os = "windows") {
        execute_terminal_command("where rustup")
    } else {
        execute_terminal_command("which rustup")
    }
}
