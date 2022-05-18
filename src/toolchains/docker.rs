use crate::cli::cli_utils::exec_stream;
use crate::ensure_tool_present;

pub fn ensure_docker_present() -> bool {
    ensure_tool_present("docker")
}


#[test]
fn test_ensure_docker_present() {
    exec_stream("docker", vec!["--version"]);
}