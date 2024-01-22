use log::info;

use crate::connection::SocketConnection;
use tungstenite::Message;

#[tauri::command]
pub async fn send_socket_message(
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
