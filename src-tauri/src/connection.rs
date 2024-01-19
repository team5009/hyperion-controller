// use url::Url;

// use std::sync::OnceLock;
// static CLIENT_HANDLE: OnceLock<> = OnceLock::new();

// pub fn get_client() -> &'static ezsockets::Client<SocketClient> {
//     CLIENT_HANDLE.get().expect("Connection not initialized")
// }

// fn set_client(handle: ezsockets::Client<SocketClient>) {
//     CLIENT_HANDLE.get_or_init(|| handle);
// }
use rust_socketio::{client::Client, ClientBuilder, Error};

pub fn connect(port: u32) -> Result<Client, Error> {
    let url = format!("http://127.0.0.1:{}", port);
    ClientBuilder::new(url).connect()
}
