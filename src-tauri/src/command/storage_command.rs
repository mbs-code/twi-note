use filesize::PathExt;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, process::Command};

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
    let filesize = match Path::new(DB_PATH).size_on_disk() {
        Ok(res) => Some(res.try_into().unwrap()),
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

#[tauri::command]
pub fn open_directory() -> bool {
    let command = if cfg!(windows) {
        "explorer"
    } else if cfg!(macos) {
        "open"
    } else {
        "xdg-open"
    };

    let dir = fs::canonicalize(DB_PATH).unwrap();
    let path = dir.parent().unwrap().to_str().unwrap();

    let res = match Command::new(command).arg(path).spawn() {
        Ok(_) => true,
        Err(_) => false,
    };
    return res;
}
