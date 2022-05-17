use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvSetupConfig {
    language: String,
    git: HashMap<String, String>,
    setup_cmds: Vec<String>
}

pub fn ingest_configuration_file(config_path: &str) -> Result<EnvSetupConfig, Box<dyn Error>> {
    let conf_str = fs::read_to_string(config_path)?;
    let result: EnvSetupConfig = serde_yaml::from_str(&conf_str)?;
    Ok(result)
}

fn validate_language(language: &str) -> bool {
    match language {
        "rust" => {
            if ensure_rustup_present() {
                println!("Found rustup!");
                return true;
            }
            println!("Rustup not found!");
            false
        },
        "python" => {
            // todo: support for python toolchain validation
            false
        },
        _ => false
    }
}

fn validate_git_conf(git_conf: &HashMap<String, String>) -> bool {
    let accepted_values = [String::from("repo"), String::from("branch")];
    let mut result = true;
    for key in git_conf.keys() {
        if !accepted_values.contains(key) {
            result = false;
        }
    }
    result
}

pub fn run(conf_path: &str) {
    let e_config = ingest_configuration_file(conf_path).unwrap_or_else(|e| {
        panic!("Error: {}", e);
    });
    process_config(&e_config);
}

fn process_config(conf: &EnvSetupConfig) {
    let language = &conf.language;
    let git_conf = &conf.git;
    let setup_cmds = &conf.setup_cmds;

    if !validate_language(language) {
        println!("Invalid language: {}", language);
        return;
    }

    if git_conf.is_empty() {
        println!("No git configuration found");
        return;
    }

    if setup_cmds.is_empty() {
        println!("No setup commands found");
        return;
    }

    let git_cmds = process_git_cmds(git_conf);
}

fn execute_cmd_list(cmd_list: &Vec<String>) -> Vec<bool> {
    let mut results = Vec::new();
    for cmd in cmd_list.iter() {
        results.push(execute_terminal_command(cmd));
    }
    results
}

fn execute_terminal_command(cmd: &str) -> bool {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(cmd).output()
    } else {
        Command::new("sh").arg("-c").arg(cmd).output()
    };
    match output {
        Ok(o) => {
            o.status.success()
        },
        Err(_) => false
    }
}


fn process_git_cmds(git_conf: &HashMap<String, String>) -> Vec<String> {
    let mut git_cmds = Vec::new();
    for (key, value) in git_conf {
        match key.as_ref() {
            "repo" => {
                let cmd = format!("git clone {}", value);
                git_cmds.push(cmd);
            },
            "branch" => {
                let cmd = format!("git checkout {}", value);
                git_cmds.push(cmd);
            },
            _ => {

            }
        }
    }
    git_cmds
}

fn ensure_rustup_present() -> bool {
    if cfg!(target_os = "windows") {
        execute_terminal_command("where rustup")
    } else {
        execute_terminal_command("which rustup")
    }
}