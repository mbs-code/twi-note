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
}

fn default_bool() -> bool {
    false
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
