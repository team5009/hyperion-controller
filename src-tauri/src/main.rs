// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hyperion_controller::construction::parse_commands;
use serde_json::{json, Value};
use std::{fs, vec};

#[tauri::command]
async fn load_file(path: String) -> Value {
    if path.is_empty() || !path.ends_with(".csv") {
        return json!({
            "error": "Invalid file"
        });
    }

    let file_content = fs::read_to_string(path).expect("Unable to read file");
    parse_commands(file_content)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
