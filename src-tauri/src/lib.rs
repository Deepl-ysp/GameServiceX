mod utils;
mod cmd;

use cmd::image::{readImage};

#[tauri::command]
fn exit() {
    std::process::exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            exit,
            readImage
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}