use crate::execute_terminal_command;
use std::collections::HashMap;

/// Checks if a directory is actually a git repository
/// # Arguments
/// * `possible_repo_dir` - The directory to check
/// # Returns
/// `true` if the directory is a git repository, `false` otherwise
pub fn check_if_in_repo(possible_repo_dir: &str) -> bool {
    let cmd = format!(
        "cd {} && git rev-parse --is-inside-work-tree",
        possible_repo_dir
    );
    execute_terminal_command(&cmd)
}

/// Processes the git commands as defined in the envsetup.yml configuration file
/// And dumps them out as a list of console-executable commands
/// Silently discards any commands that are not supported
/// # Arguments
/// * `git_conf` - The git configuration after parsing envsetup.yml
/// # Returns
/// A list of console-executable commands
///
pub fn process_git_cmds(git_conf: &HashMap<String, String>) -> Vec<String> {
    let mut git_cmds = Vec::new();
    for (key, value) in git_conf {
        match key.as_ref() {
            "repo" => {
                let cmd = format!("git clone {}", value);
                git_cmds.push(cmd);
            }
            "branch" => {
                let cmd = format!("git checkout {}", value);
                git_cmds.push(cmd);
            }
            _ => {}
        }
    }
    git_cmds
}
