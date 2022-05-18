use std::process::{Command, Stdio};
use std::path::Path;
use std::io::{BufReader, BufRead};

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
pub fn execute_terminal_command(cmd: &str) -> bool {
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

/// Checks to ensure that some CLI tool is present
/// This function essentially runs which <tool>
pub fn ensure_tool_present(tool: &str) -> bool {
    if cfg!(target_os = "windows") {
        execute_terminal_command(format!("where {}", tool).as_str())
    } else {
        execute_terminal_command(format!("which {}", tool).as_str())
    }
}

pub fn exec_stream<P: AsRef<Path>>(binary: P, args: Vec<&'static str>) {
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
                println!("> {}", l);
            }
            println!();
        }
    }

    cmd.wait().unwrap();
}


#[test]

fn test_exec_stream() {
    exec_stream("apt", vec!["search", "rust"]);
}