// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use hyperion_controller::connection::{connect, get_client};
use hyperion_controller::connection::{connect, SocketConnection};
use hyperion_controller::ConnectionStatus;
use hyperion_controller::{construction::parse_commands, math::bezier_curve, Call, Point};
use log::{error, info};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use tauri::{Manager, State};
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;

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
            println!("Connected to bot");
            *websocket_client.0.lock().unwrap() = Some(client.0);
            app.emit_all("bot_connect", {
                json!({
                    "status": ConnectionStatus::Connected as u16
                })
            })
            .unwrap();
        }
        Err(e) => {
            println!("Error connecting to bot: {}", e);
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
fn disconnect_bot(app: tauri::AppHandle, websocket_client: tauri::State<'_, SocketConnection>) {
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
}

#[tauri::command]
async fn send_socket_message(
    app: tauri::AppHandle,
    message: String,
    websocket_client: tauri::State<'_, SocketConnection>,
) -> Result<(), ()> {
    let mut client = websocket_client.0.lock().unwrap();
    if client.is_none() {
        return Err(());
    }
    let client = client.as_mut().unwrap();

    let message = tungstenite::Message::Text(message);

    info!("Sending message: {}", message);
    client.send(message).unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.app_handle();
            thread::spawn(move || loop {
                let socket_state: State<SocketConnection> = app_handle.state();
                let mut socket = socket_state.0.lock().unwrap();
                let socket = socket.as_mut();
                if socket.is_none() {
                    continue;
                }

                match socket.unwrap().read() {
                    Ok(msg) => {
                        let msg = msg.to_text().unwrap();

                        info!("Received message: {}", msg);
                        app_handle.emit_all("bot_event", msg).unwrap();
                    }
                    Err(e) => {
                        error!("Error reading from socket: {}", e);
                        app_handle
                            .emit_all("bot_connect", {
                                json!({
                                    "status": ConnectionStatus::Error as u16,
                                    "message": "Error reading from socket"
                                })
                            })
                            .unwrap();
                        *app_handle
                            .state::<State<SocketConnection>>()
                            .0
                            .lock()
                            .unwrap() = None;
                        break;
                    }
                }
            });

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
