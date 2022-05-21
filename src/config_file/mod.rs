use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type LanguageOptions = HashMap<String, String>;
type GitOptions = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvSetupConfig {
    pub language: String,
    pub language_options: Option<LanguageOptions>,
    pub git: GitOptions,
    pub setup_cmds: Option<Vec<String>>,
    pub container_system: Option<String>,
}
