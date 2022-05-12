use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_as_false")]
    pub is_dark: bool,
    #[serde(default = "default_as_false")]
    pub expand_side: bool,
    #[serde(default = "default_as_false")]
    pub expand_editor: bool,

    #[serde(default = "default_mode_relative")]
    pub timestamp_mode: String,
    #[serde(default = "default_as_false")]
    pub use_updated_at: bool,
    #[serde(default = "default_as_false")]
    pub hide_edited: bool,

    #[serde(default = "default_as_20")]
    pub tl_once_count: i64,
    #[serde(default = "default_as_0")]
    pub timezone_offset_sec: i64,
    #[serde(default = "default_as_0_0")]
    pub start_of_day: String,

    #[serde(default = "default_as_false")]
    pub use_phrase: bool,
}

fn default_as_false() -> bool {
    false
}

fn default_mode_relative() -> String {
    "relative".to_string()
}

fn default_as_0() -> i64 {
    0
}

fn default_as_20() -> i64 {
    20
}

fn default_as_0_0() -> String {
    "00:00".to_string()
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
