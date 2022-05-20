use crate::cli::cli_utils::exec_stream;
use crate::ensure_tool_present;

/// Checks if docker is installed
/// This will fail if docker is installed but not in PATH
pub fn ensure_docker_present() -> bool {
    ensure_tool_present("docker")
}


#[test]
fn test_ensure_docker_present() {
    exec_stream("docker", vec!["--version"]);
}