use serde_json::json;
use tauri::{AppHandle, Manager};
use tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};

use crate::{connection::SocketConnection, ConnectionStatus};

#[tauri::command]
pub async fn disconnect_bot(
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
