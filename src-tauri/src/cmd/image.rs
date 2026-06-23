// src-tauri/src/cmd/image.rs
use std::path::PathBuf;
use crate::utils::fileAPI::{FileObject, readFile};
use crate::utils::ToTauriError;
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

    let fileObject: FileObject = readFile(&app, &resPath)
        .map_err(|e| e.to_tauri_error())?;

    let baseString: String = STANDARD.encode(&fileObject.data);

    Ok(ImageObject {
        name: fileObject.name,
        path: fileObject.path,
        data: baseString,
        types: "image".to_string(),
    })
}