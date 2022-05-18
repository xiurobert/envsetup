use crate::ensure_tool_present;

pub fn ensure_docker_present() -> bool {
    ensure_tool_present("docker")
}
