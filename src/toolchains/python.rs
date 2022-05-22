use crate::cli::cli_utils::exec_stream;
use crate::{ensure_tool_present, execute_terminal_command};
use crate::os::check_aptget_present;

/// Checks if python3 is installed
/// This will fail if python is installed but not in PATH
pub fn ensure_python_present() -> bool {
    ensure_tool_present("python3")
}

pub enum PythonVersion {
    Python3_10,
    Python3_9,
    Python3_8,
    Python3_7,
}

pub enum PyToolchain {
    Poetry,
    Virtualenv,
    Pipenv
}

pub fn initial_setup(toolchain: &PyToolchain) -> bool {
    match toolchain {
        PyToolchain::Poetry => {
            println!("Executing poetry install...");
            exec_stream("poetry", vec!["install"], true)
        },
        PyToolchain::Virtualenv => {
            println!("Creating virtualenv...");
            exec_stream("virtualenv", vec!["-p", "python3", ".venv"], true);
            exec_stream(".venv/bin/pip", vec!["install", "-r", "requirements.txt"], true)
        },
        PyToolchain::Pipenv => {
            println!("Creating pipenv...");
            exec_stream("pipenv", vec!["install"], true)
        }
    }
}

pub fn install_python(python_version: &PythonVersion) -> bool {
    if cfg!(target = "macos") {
        return install_python_macos(python_version);
    } else if cfg!(target = "windows") {
        return install_python_windows(python_version);
    } else if cfg!(target = "linux") {
        return install_python_linux(python_version);
    }
    false
}

fn install_python_windows(python_version: &PythonVersion) -> bool {
    let install_arg = format!("python-{}", calc_ver(python_version));
    exec_stream("choco", vec!["install", install_arg.as_str()], true)
}

fn install_python_linux(python_version: &PythonVersion) -> bool {
    let package = format!("python{}", calc_ver(python_version));
    linux_add_deadsnakes_ppa();
    if check_aptget_present() {
        return exec_stream("apt-get", vec!["install", package.as_str()], true);
    }
    false
}

fn linux_add_deadsnakes_ppa() -> bool {
    if !ensure_tool_present("add-apt-repository") {
        return exec_stream("add-apt-repository", vec!["ppa:deadsnakes/ppa"], true);
    }
    true
}

fn calc_ver(python_version: &PythonVersion) -> &str {
    match python_version {
        PythonVersion::Python3_10 => "3.10",
        PythonVersion::Python3_9 => "3.9",
        PythonVersion::Python3_8 => "3.8",
        PythonVersion::Python3_7 => "3.7",
    }
}

fn install_python_macos(python_version: &PythonVersion) -> bool {
    let install_arg = format!("python@{}", calc_ver(python_version));
    exec_stream("brew", vec!["install", install_arg.as_str()], true)
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

#[cfg(test)]
mod tests {
    use crate::toolchains::python::calc_ver;

    #[test]
    fn calc_ver_is_correct() {
        assert_eq!(calc_ver(&super::PythonVersion::Python3_10), "3.10");
        assert_eq!(calc_ver(&super::PythonVersion::Python3_9), "3.9");
        assert_eq!(calc_ver(&super::PythonVersion::Python3_8), "3.8");
        assert_eq!(calc_ver(&super::PythonVersion::Python3_7), "3.7");

    }
}
