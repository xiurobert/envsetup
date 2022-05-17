use std::process::Command;

/// Execute a list of commands
pub fn execute_cmd_list(cmd_list: &[String]) -> Vec<bool> {
    let mut results = Vec::new();
    for cmd in cmd_list.iter() {
        results.push(execute_terminal_command(cmd));
    }
    results
}

/// Execute a command "in the terminal"
/// This function tries to emulate the experience of typing the command in your terminal
/// and pressing enter.
pub(crate) fn execute_terminal_command(cmd: &str) -> bool {
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
