use serde::{Deserialize, Serialize};


// pub type LanguageOptions = HashMap<String, String>;
// pub type GitOptions = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Python
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvSetupConfig {
    pub language: Language,
    pub language_options: Option<LanguageOptions>,
    pub git: GitOptions,
    pub setup_cmds: Option<Vec<String>>,
    pub container_system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageOptions {
    pub version: String,
    pub toolchain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitOptions {
    pub repo: String,
    pub branch: String,
}