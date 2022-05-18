use crate::cli_utils::{ensure_tool_present, execute_terminal_command};

/// Checks if python3 is installed
pub fn ensure_python_present() -> bool {
    ensure_tool_present("python3")
}

/// Checks if python poetry is installed
pub fn ensure_poetry_present() -> bool {
    ensure_tool_present("poetry")
}
