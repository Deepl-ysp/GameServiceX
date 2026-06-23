use std::path::PathBuf;
use crate::utils::fileAPI::{FileObject, readFile};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use tauri::AppHandle;

#[derive(serde::Serialize)]
pub struct ImageObject {
    name: String,
    path: PathBuf,
    data: String,
    types: String,
}

#[allow(non_snake_case)]
#[tauri::command]
pub fn readImage(app: AppHandle, resPath: String) -> Result<ImageObject, String> {
    let fileObject:FileObject = readFile(&app, &resPath).map_err(|e| format!("Failed to read file: {}", e))?;

    let fileData: Vec<u8> = fileObject.data;

    let baseString: String = STANDARD.encode(&fileData);

    let pathBuf: PathBuf = PathBuf::from(&resPath);

    let name: String = pathBuf
        .file_name()
        .and_then(|n: &std::ffi::OsStr| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(ImageObject {
        name,
        path: pathBuf,
        data: baseString,
        types: "image".to_string()
    })
}
