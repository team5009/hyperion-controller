// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use hyperion_controller::connection::connect;
use hyperion_controller::controller::support_controller;
use hyperion_controller::misc::socket_handle;
use hyperion_controller::{
    connection::SocketConnection, construction::command_read, math::bezier_curve, ConnectionStatus,
    Point,
};
use log::{error, info};
use serde_json::{json, Value};
use std::fs;
use tauri::{AppHandle, Manager, State};
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::Message;

#[tauri::command]
async fn connect_to_bot(
    app: AppHandle,
    port: u32,
    websocket_client: tauri::State<'_, SocketConnection>,
) -> Result<(), ()> {
    app.emit_all("bot_connect", {
        json!({
            "status": ConnectionStatus::Pending as u16,
            "message": "Connecting to bot..."
        })
    })
    .unwrap();

    match connect(port) {
        Ok(client) => {
            info!("Connected to bot");
            *websocket_client.0.lock().unwrap() = Some(client.0);
            app.emit_all("bot_connect", {
                json!({
                    "status": ConnectionStatus::Connected as u16
                })
            })
            .unwrap();
        }
        Err(e) => {
            error!("Error connecting to bot: {}", e);
            app.emit_all("bot_connect", {
                json!({
                    "status": ConnectionStatus::Error as u16,
                    "message": "Unable to connect to bot"
                })
            })
            .unwrap();
        }
    }

    Ok(())
}

#[tauri::command]
async fn disconnect_bot(
    app: AppHandle,
    websocket_client: tauri::State<'_, SocketConnection>,
) -> Result<(), ()> {
    websocket_client
        .0
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .close(
            CloseFrame {
                code: CloseCode::Normal,
                reason: std::borrow::Cow::Borrowed(""),
            }
            .into(),
        )
        .unwrap();
    *websocket_client.0.lock().unwrap() = None;

    app.emit_all("bot_connect", {
        json!({
            "status": ConnectionStatus::Disconnected as u16
        })
    })
    .unwrap();

    Ok(())
}

#[tauri::command]
async fn send_socket_message(
    message: String,
    websocket_client: tauri::State<'_, SocketConnection>,
) -> Result<(), ()> {
    let mut client = websocket_client.0.lock().unwrap();
    if client.is_none() {
        return Err(());
    }
    let client = client.as_mut().unwrap();

    let message = Message::Text(message);

    info!("Sending message: {}", message);
    client.send(message).unwrap();

    Ok(())
}

#[tauri::command]
async fn load_file(path: String) -> Value {
    if path.is_empty() || !path.ends_with(".csv") {
        return json!({
            "error": "Invalid file"
        });
    }

    let file_content = fs::read_to_string(path).expect("Unable to read file");
    command_read(file_content).unwrap()
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

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::spawn(socket_handle(app.app_handle()));
            tauri::async_runtime::spawn(support_controller(app.app_handle()));
            Ok(())
        })
        .manage(SocketConnection(Default::default()))
        .invoke_handler(tauri::generate_handler![
            load_file,
            gen_bezier_points,
            connect_to_bot,
            disconnect_bot,
            send_socket_message
        ])
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
