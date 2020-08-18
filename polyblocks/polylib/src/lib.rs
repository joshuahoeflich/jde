extern crate tokio;

use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

#[derive(Debug)]
pub enum PolyblocksError {
    SocketConnection,
    SocketWrite,
}

pub async fn write_polyblocks<'a>(
    socket_addr: &'a str,
    block: &'a str,
    output: &'a str,
) -> Result<(), PolyblocksError> {
    let mut stream = UnixStream::connect(socket_addr)
        .await
        .map_err(|_| PolyblocksError::SocketConnection)?;
    let socket_bytes = format!("{}\n{}", block, output).into_bytes();
    stream
        .write(&socket_bytes)
        .await
        .map_err(|_| PolyblocksError::SocketWrite)
        .map(|_| ())
}

pub fn render_pbwrite_error(err: PolyblocksError) {
    match err {
        PolyblocksError::SocketConnection => eprintln!("Could not connect to socket"),
        PolyblocksError::SocketWrite => eprintln!("Could not write to socket"),
    }
}
