use crate::cli_utils::ensure_tool_present;

/// Checks if docker is installed
/// This will fail if docker is installed but not in PATH
pub fn ensure_docker_present() -> bool {
    ensure_tool_present("docker")
}
