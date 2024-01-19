// use url::Url;

use std::sync::{OnceLock, Mutex};
use tungstenite::{
    connect as socket_connect, http::Response, stream::MaybeTlsStream, Error, WebSocket,
};
use url::Url;
use std::net::TcpStream;

// pub fn get_client() -> &'static ezsockets::Client<SocketClient> {
//     CLIENT_HANDLE.get().expect("Connection not initialized")
// }

// fn set_client(handle: ezsockets::Client<SocketClient>) {
//     CLIENT_HANDLE.get_or_init(|| handle);
// }


static SOCKET_CLIENT: Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>> = Mutex::new(None);
static SOCKET_RESPONSE: OnceLock<Response<Option<Vec<u8>>>> = OnceLock::new();
static CLIENT_CONNECTED: OnceLock<bool> = OnceLock::new();

pub struct SocketResponse {
    inner: (
        WebSocket<MaybeTlsStream<TcpStream>>,
        Response<Option<Vec<u8>>>,
    ),
}

pub fn connect(port: u32) -> Result<(), ()> {
    let url_string = format!("http://127.0.0.1:{}", port);
    let url = Url::parse(&url_string).unwrap();
    match socket_connect(url) {
        Ok(connection) => {
            let (socket, response) = connection;
            match SOCKET_CLIENT.lock() {
                Ok(client) => client = socket,
                Err()
            }
            Ok(())
        }
        Err(e) => {
            println!("{:?}", e);
            Err(())
        }
    }
    // ClientBuilder::new(url).connect()
}
