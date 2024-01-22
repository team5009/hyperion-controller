use serde_json::json;
use tauri::{AppHandle, Manager, State};

use crate::{connection::SocketConnection, ConnectionStatus};

pub async fn socket_handle(
    handle: AppHandle,
) -> Result<(), std::sync::PoisonError<SocketConnection>> {
    let socket_state: State<SocketConnection> = handle.state::<SocketConnection>();
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        let mut socket = match socket_state.0.lock() {
            Ok(socket) => socket,
            Err(_e) => {
                return Ok(());
            }
        };

        if socket.is_none() {
            continue;
        };

        match socket.as_mut().unwrap().read() {
            Ok(msg) => {
                let msg = msg.to_text().unwrap();
                handle.emit_all("bot_event", msg).unwrap();
            }
            Err(_e) => {
                *socket_state.0.lock().unwrap() = None;

                handle
                    .emit_all("bot_connect", {
                        json!({
                            "status": ConnectionStatus::Disconnected as u16
                        })
                    })
                    .unwrap();

                continue;
            }
        };
    }
    // Ok(())
}
