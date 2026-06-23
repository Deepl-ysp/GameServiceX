use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tauri::Manager;
use tauri::path::BaseDirectory;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FileObject {
    pub name: String,
    pub path: PathBuf,
    pub data: Vec<u8>,
}

#[allow(non_snake_case)]
pub fn readFile(app: &tauri::AppHandle, resPath: &str) -> Result<FileObject, String> {
    let real_path: PathBuf = app
        .path()
        .resolve(resPath, BaseDirectory::Resource)
        .map_err(|e: tauri::Error| format!("解析资源路径失败: {}", e))?;

    let data: Vec<u8> = std::fs::read(&real_path)
        .map_err(|e: std::io::Error| format!("读取文件失败: {}", e))?;

    let name: String = real_path
        .file_name()
        .and_then(|os: &std::ffi::OsStr| os.to_str())
        .map(String::from)
        .unwrap_or_else(|| "unknown".to_string());

    Ok(FileObject {
        name,
        path: real_path,
        data,
    })
}