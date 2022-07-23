#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use crate::request::send_request;
mod request;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, send_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
