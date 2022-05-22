use crate::{
    ensure_docker_present, process_language,
    EnvSetupConfig,
};


/// Performs validation on the EnvSetupConfig struct
/// Returns a boolean based on the validity of the configuration
pub fn validate_config(conf: &EnvSetupConfig) -> bool {
    let language = &conf.language;
    // let language_opts = &conf.language_options;
    // let setup_cmds = &conf.setup_cmds;
    let container_system = &conf.container_system;

    if !process_language(language) {
        println!("Could not process language: {:?}", language);
        return false;
    }

    println!("Processed default commands for language: {:?}", language);

    if let Some(container_system) = container_system {
        if !validate_container_system(container_system) {
            println!("Container system is not supported");
            return false;
        }
    }
    true
}

/// Validates the container system attribute in the configuration file
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
