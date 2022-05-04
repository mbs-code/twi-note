use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_bool")]
    pub is_dark: bool,
    #[serde(default = "default_bool")]
    pub expand_side: bool,
    #[serde(default = "default_bool")]
    pub expand_editor: bool,

    #[serde(default = "default_timestamp_mode")]
    pub timestamp_mode: String,
    #[serde(default = "default_bool")]
    pub ref_updated_at: bool,
    #[serde(default = "default_count")]
    pub tl_once_count: i64,
}

fn default_bool() -> bool {
    false
}

fn default_timestamp_mode() -> String {
    "relative".to_string()
}

fn default_count() -> i64 {
    20
}

///

#[tauri::command]
pub fn load_config() -> Option<AppConfig> {
    let yaml = fs::read_to_string("./config.yaml").unwrap_or("".to_string());

    let config: Option<AppConfig> = match serde_yaml::from_str(&yaml) {
        Ok(config) => Some(config),
        Err(_) => None,
    };
    return config;
}

#[tauri::command]
pub fn save_config(config: AppConfig) -> bool {
    let yaml = serde_yaml::to_string(&config).unwrap();

    let res = match fs::write("./config.yaml", &yaml) {
        Ok(()) => true,
        Err(_) => false,
    };
    return res;
}
