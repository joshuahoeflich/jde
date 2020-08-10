use super::cli::InputData;
use super::errors::error_message;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::stream::StreamExt;

fn output(status: String, append_newlines: bool) {
    if append_newlines {
        println!("{}", status);
        return;
    }
    print!("{}", status);
}

pub async fn create_server(
    input_data: &mut InputData,
    running: Arc<AtomicBool>,
) -> Result<(), ServerSetup> {
    let mut listener = tokio::net::UnixListener::bind(&input_data.socket_addr)
        .map_err(|_| ServerSetup::SocketConnection)?;

    output(input_data.cache.status(), input_data.append_newlines);

    while let Some(stream) = listener.next().await {
        let should_listen = running.load(Ordering::SeqCst);
        if !should_listen {
            break;
        }
        match stream {
            Ok(mut stream) => match parse_stream(&mut stream).await {
                Ok(bar_item) => {
                    input_data.cache.update(bar_item.key, bar_item.value);
                    output(input_data.cache.status(), input_data.append_newlines);
                }
                Err(e) => eprint!("ERR {:?}", e),
            },
            Err(e) => eprint!("ERR {:?}", e),
        }
    }

    fs::remove_file(&input_data.socket_addr).map_err(|_| ServerSetup::RemoveSocketFile)?;
    Ok(())
}

#[derive(Debug)]
enum ServerRuntime {
    StreamRead,
    StringParse,
}

struct BarUpdate {
    key: String,
    value: String,
}

async fn parse_stream(stream: &mut tokio::net::UnixStream) -> Result<BarUpdate, ServerRuntime> {
    let mut buffer = Vec::new();
    stream
        .read_to_end(&mut buffer)
        .await
        .map_err(|_| ServerRuntime::StreamRead)?;
    let buf_string = std::str::from_utf8(&buffer).map_err(|_| ServerRuntime::StringParse)?;
    let mut update = BarUpdate {
        key: String::new(),
        value: String::new(),
    };
    for (counter, line) in buf_string.lines().enumerate() {
        match counter {
            0 => update.key.push_str(&line.to_lowercase()),
            1 => update.value.push_str(line),
            _ => break,
        }
    }
    Ok(update)
}

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
