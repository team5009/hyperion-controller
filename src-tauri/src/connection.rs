// use url::Url;

use std::net::TcpStream;
use std::sync::Mutex;
use tungstenite::{
    connect as socket_connect, http::Response, stream::MaybeTlsStream, Error, WebSocket,
};
use url::Url;

pub struct SocketConnection(pub Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>);
pub struct SocketResponse(pub Mutex<Option<Response<Option<Vec<u8>>>>>);

pub fn connect(
    port: u32,
) -> Result<
    (
        WebSocket<MaybeTlsStream<TcpStream>>,
        Response<Option<Vec<u8>>>,
    ),
    Error,
> {
    let url_string = format!("ws://127.0.0.1:{}", port);
    let url = Url::parse(&url_string).unwrap();
    socket_connect(url)
}
