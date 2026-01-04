use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub api_key: Option<String>,
    pub url: Option<String>,
    pub model: Option<String>,
    pub language: Option<String>,
    pub prompt: Option<String>,
}

impl Config {
    pub fn save_config(&self) {
        let config_path = get_config_path();
        let config_str = toml::to_string_pretty(&self).expect("Failed to serialize config");
        fs::write(config_path, config_str).expect("Failed to write config file");
    }
}
pub fn load_config() -> Config {
    let config_path = get_config_path();
    if !config_path.exists() {
        return Config::default();
    }
    let config_str = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&config_str).expect("Failed to parse config file")
}

pub fn get_config_path() -> PathBuf {
    let proj_dirs =
        ProjectDirs::from("com", "github", "ai-commit").expect("Failed to get project directories");
    let config_dir = proj_dirs.config_dir();
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Failed to create config directory");
    }
    config_dir.join("config.toml")
}
