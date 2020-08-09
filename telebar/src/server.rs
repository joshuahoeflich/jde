use super::cli::InputData;
use super::errors::error_message;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
// use tokio::io::AsyncReadExt;
use tokio::stream::StreamExt;

pub async fn create_server(
    input_data: &mut InputData,
    running: Arc<AtomicBool>,
) -> Result<(), ServerSetup> {
    let mut listener = tokio::net::UnixListener::bind(&input_data.socket_addr)
        .map_err(|_| ServerSetup::SocketConnection)?;

    while let Some(stream) = listener.next().await {
        let should_listen = running.load(Ordering::SeqCst);
        if !should_listen {
            break;
        }
        // stream
        //     .map(parse_stream)
        //     .or_else(|runtime_err| Err(runtime_err));
        // match stream {
        //     Ok(mut stream) => {
        //         parse_stream(&mut stream).await
        //     },
        //     Err(e) => eprintln!("ERR {:?}", e),
        // }
    }
    fs::remove_file(&input_data.socket_addr).map_err(|_| ServerSetup::RemoveSocketFile)?;
    Ok(())
}

// async fn parse_stream(stream: &mut tokio::net::UnixStream) -> Result<String, ()> {
//     let mut buffer = Vec::new();
//     stream.read_to_end(&mut buffer).await.unwrap();
//     match std::str::from_utf8(&buffer) {
//         Ok(string) => println!("YOU SENT {}", string),
//         Err(e) => eprintln!("{:?}", e),
//     };
// }

pub enum ServerSetup {
    SocketConnection,
    RemoveSocketFile,
}

pub fn suggest_server_fix(error: ServerSetup, socket_addr: &str) {
    let path = socket_addr.to_owned();
    match error {
        ServerSetup::SocketConnection => error_message(
            "COULD NOT CONNECT TO SOCKET",
            format!("Try deleting the file {}", path),
        ),
        ServerSetup::RemoveSocketFile => error_message(
            "COULD NOT DELETE SOCKET FILE",
            format!("Please delete the file {}", path),
        ),
    }
}
