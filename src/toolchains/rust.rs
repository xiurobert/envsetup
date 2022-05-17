use crate::cli_utils::execute_terminal_command;

/// Checks if rustup is installed
pub fn ensure_rustup_present() -> bool {
    if cfg!(target_os = "windows") {
        execute_terminal_command("where rustup")
    } else {
        execute_terminal_command("which rustup")
    }
}
