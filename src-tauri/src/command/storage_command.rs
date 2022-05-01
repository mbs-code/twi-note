use serde::{Deserialize, Serialize};
use std::fs;
use std::os::windows::fs::MetadataExt;
use std::path::Path;

use crate::DB_PATH;

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageInfo {
    pub path: String,
    pub exist: bool,
    pub size: Option<i64>,
}

///

#[tauri::command]
pub fn get_storage_info() -> StorageInfo {
    let filesize = match fs::metadata(DB_PATH) {
        Ok(res) => Some(res.file_size().try_into().unwrap()),
        Err(_) => None,
    };

    let filepath: String = match dunce::canonicalize(DB_PATH) {
        Ok(res) => res.into_os_string().into_string().unwrap(),
        Err(_) => "".to_string(),
    };

    return StorageInfo {
        path: filepath,
        exist: if filesize.is_some() { true } else { false },
        size: filesize,
    };
}