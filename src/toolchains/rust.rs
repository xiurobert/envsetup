use crate::cli::cli_utils::ensure_tool_present;

/// Checks if rustup is installed
pub fn ensure_rustup_present() -> bool {
    ensure_tool_present("rustup")
}
