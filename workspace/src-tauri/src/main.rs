#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::db::{create_request_record, list_all_requests, update_request_record};
use crate::request::send_request;
mod db;
mod prisma;
mod request;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            send_request,
            create_request_record,
            update_request_record,
            list_all_requests
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
