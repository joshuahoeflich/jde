use super::errors::error_message;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::stream::StreamExt;

pub async fn create_server(socket_addr: &str, running: Arc<AtomicBool>) -> Result<(), ServerError> {
    let mut listener = tokio::net::UnixListener::bind(socket_addr)
        .map_err(|_| ServerError::SocketConnectionError)?;

    while let Some(stream) = listener.next().await {
        let should_listen = running.load(Ordering::SeqCst);
        if !should_listen {
            break;
        }
        match stream {
            Ok(mut stream) => handle_stream(&mut stream).await,
            Err(e) => eprintln!("ERR {:?}", e),
        }
    }
    fs::remove_file(socket_addr).map_err(|_| ServerError::RemoveSocketFileError)?;
    Ok(())
}

async fn handle_stream(stream: &mut tokio::net::UnixStream) {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await.unwrap();
    match std::str::from_utf8(&buffer) {
        Ok(string) => println!("YOU SENT {}", string),
        Err(e) => eprintln!("{:?}", e),
    };
}

pub fn get_socket_addr(server_id: &str) -> Result<String, std::env::VarError> {
    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR")?;
    let mut socket_buffer = PathBuf::new();
    socket_buffer.push(xdg_runtime_dir);
    socket_buffer.push(format!("{}_telebar_socket", server_id));
    Ok(socket_buffer.to_string_lossy().into_owned())
}

pub enum ServerError {
    SocketConnectionError,
    RemoveSocketFileError,
}

pub fn suggest_server_fix(error: ServerError, socket_addr: &str) {
    let path = socket_addr.to_owned();
    match error {
        ServerError::SocketConnectionError => error_message(
            "COULD NOT CONNECT TO SOCKET",
            format!("Try deleting the file {}", path),
        ),
        ServerError::RemoveSocketFileError => error_message(
            "COULD NOT DELETE SOCKET FILE",
            format!("Please delete the file {}", path),
        ),
    }
}
