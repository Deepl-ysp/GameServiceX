use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tauri::{Manager, path::BaseDirectory};
use crate::utils::error::GsResult;
use anyhow::Context;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct FileObject {
    pub name: String,
    pub path: PathBuf,
    pub data: Vec<u8>,
}

#[allow(non_snake_case)]
pub fn readFile(app: &tauri::AppHandle, resPath: &str) -> GsResult<FileObject> {

    let real_path: PathBuf = app
        .path()
        .resolve(resPath, BaseDirectory::Resource)
        .context(format!("解析 Resource 路径失败: {}", resPath))?;


    if !real_path.exists() {
        log::error!("【readFile】文件不存在: {:?}", real_path);
        return Err(anyhow::anyhow!("资源文件不存在: {}", resPath));
    }

    let data: Vec<u8> = std::fs::read(&real_path)
        .with_context(|| format!("读取文件失败: {}", resPath))?;

    let name: String = real_path
        .file_name()
        .and_then(|os| os.to_str())
        .map(String::from)
        .unwrap_or_else(|| "unknown".to_string());

    Ok(FileObject { name, path: real_path, data })
}