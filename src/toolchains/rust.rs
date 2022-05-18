use crate::cli_utils::{ensure_tool_present, execute_terminal_command};

/// Checks if rustup is installed
pub fn ensure_rustup_present() -> bool {
    ensure_tool_present("rustup")
}
