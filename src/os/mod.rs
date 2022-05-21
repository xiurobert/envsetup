use crate::ensure_tool_present;
use std::env;

pub fn check_homebrew_present() -> bool {
    cfg!(target_os = "macos") && ensure_tool_present("brew")
}

pub fn check_aptget_present() -> bool {
    cfg!(target_os = "linux") && ensure_tool_present("apt-get")
}

pub fn check_choco_present() -> bool {
    cfg!(target_os = "windows") && ensure_tool_present("choco")
}
