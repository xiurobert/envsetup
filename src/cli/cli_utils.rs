use std::error::Error;
use std::process::{Command, Output, Stdio};
use std::path::Path;
use std::io::{BufReader, BufRead};

/// Execute a list of commands
pub fn execute_cmd_list(cmd_list: &[String]) -> Vec<bool> {
    let mut results = Vec::new();
    for cmd in cmd_list.iter() {
        results.push(execute_terminal_command(cmd).is_ok());
    }
    results
}

/// Execute a command "in the terminal"
/// This function tries to emulate the experience of typing the command in your terminal
/// and pressing enter.
/// # Arguments
/// * `cmd` - The command to execute
/// # Returns
/// `Ok(Output)` if the command was executed successfully, `Err(Box<Error>)` if the command failed
pub fn execute_terminal_command(cmd: &str) -> Result<Output, Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(cmd).output()
    } else {
        Command::new("sh").arg("-c").arg(cmd).output()
    };
    match output {
        Ok(o) => Ok(o),
        Err(e) => Err(Box::new(e)),
    }
}

/// Checks to ensure that some CLI tool is present
/// This function essentially runs which <tool>
/// # Arguments
/// * `tool` - The name of the tool to check for
/// # Returns
/// `true` if the tool is present, `false` otherwise
pub fn ensure_tool_present(tool: &str) -> bool {
    if cfg!(target_os = "windows") {
        execute_terminal_command(format!("where {}", tool).as_str()).is_ok()
    } else {
        execute_terminal_command(format!("which {}", tool).as_str()).is_ok()
    }
}

/// Executes a command in the terminal and streams the output directly to stdout
/// # Arguments
/// * `binary` - The program to execute
/// * `args` - The arguments to pass to the program
/// # Returns
/// `true` if the command was executed successfully, `false` otherwise
pub fn exec_stream<P: AsRef<Path>>(binary: P, args: Vec<&'static str>) -> bool {
    let mut cmd = Command::new(binary.as_ref())
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            if let Ok(l) = line {
                println!("{}", l);
            }
            println!();
        }
    }

    let exit_status = cmd.wait().unwrap();
    exit_status.success()
}
