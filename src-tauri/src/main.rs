// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use hyperion_controller::connection::{connect, get_client};
use hyperion_controller::connection::connect;
use hyperion_controller::ConnectionStatus;
use hyperion_controller::{construction::parse_commands, math::bezier_curve, Call, Point};
use serde_json::{json, Value};
use std::fs;
use std::sync::{Arc, Mutex};
use tauri::Manager;

struct SocketClient;

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

#[tauri::command]
fn gen_bezier_points(points: Vec<Point>, resolution: u32) -> Value {
    if points.len() != 4 {
        return json!({
            "error": "Invalid number of points"
        });
    }
    bezier_curve(points, resolution)
}

#[tauri::command]
async fn connect_to_bot(
    app: tauri::AppHandle,
    port: u32,
    websocket_client: tauri::State<'_, SocketClient>,
) -> Result<(), ()> {
    app.emit_all("bot_connect", {
        json!({
            "status": ConnectionStatus::Pending as u16,
            "message": "Connecting to bot..."
        })
    })
    .unwrap();

    let client = connect(port);

    match client {
        Ok(client) => {
            app.emit_all("bot_connect", {
                json!({
                    "status": ConnectionStatus::Connected as u16
                })
            })
            .unwrap();
        }
        Err(e) => {
            app.emit_all("bot_connect", {
                json!({
                    "status": ConnectionStatus::Error as u16,
                    "message": "Unable to connect to bot"
                })
            })
            .unwrap();

            println!("Error: {}", e)
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle: Arc<Mutex<tauri::AppHandle>> = Arc::new(Mutex::new(app.handle()));

            Ok(())
        })
        .manage(SocketClient {})
        .invoke_handler(tauri::generate_handler![
            load_file,
            gen_bezier_points,
            connect_to_bot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
