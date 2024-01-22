use log::{error, info};
use serde_json::json;
use tauri::{AppHandle, Manager};
use tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

use crate::{
    connection::{connect, SocketConnection},
    ConnectionStatus,
};

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
