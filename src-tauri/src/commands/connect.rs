use log::{error, info};
use serde_json::json;
use tauri::{AppHandle, Manager};
use tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};

use crate::{
    connection::{connect, SocketConnection},
    ConnectionStatus,
};

#[tauri::command]
pub async fn connect_to_bot(
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
