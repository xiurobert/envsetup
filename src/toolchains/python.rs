use crate::ensure_tool_present;

/// Checks if python3 is installed
/// This will fail if python is installed but not in PATH
pub fn ensure_python_present() -> bool {
    ensure_tool_present("python3")
}

/// Checks if python3-virtualenv is available
/// This will fail if venv is installed but not in PATH
pub fn ensure_virtualenv() -> bool {
    ensure_tool_present("venv")
}

/// Checks if python poetry is installed
/// Note this tool only checks if the poetry command is accessible
/// If poetry is not in PATH, it will return false.
/// # Returns
/// `true` if installed, `false` otherwise
pub fn ensure_poetry_present() -> bool {
    ensure_tool_present("poetry")
}
