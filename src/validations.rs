use crate::{ensure_docker_present, ensure_python_present, ensure_rustup_present, EnvSetupConfig, process_language, validate_container_system, validate_git_conf};

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

/// Performs validation on the EnvSetupConfig struct
/// Returns a boolean based on the validity of the configuration
pub fn validate_config(conf: &EnvSetupConfig) -> bool {
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

/// Validates the container system attribute in the configuration file
pub fn validate_container_system(container_system: &str) -> bool {
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