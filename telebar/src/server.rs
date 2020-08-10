use super::cli::{InputData, OutputFormat};
use super::errors::error_message;
use std::convert::TryFrom;
use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::stream::StreamExt;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, PropMode};
use x11rb::wrapper::ConnectionExt;

#[derive(Debug)]
enum XSetRoot {
    ConnectionFailed,
    TryFromU32Failure,
    PaintingError,
    ConSyncError,
}

fn xsetroot(status: String) {
    match x11rb::connect(None)
        .map_err(|_| XSetRoot::ConnectionFailed)
        .and_then(|(conn, screen_num)| {
            u32::try_from(status.chars().count())
                .map_err(|_| XSetRoot::TryFromU32Failure)
                .map(|strlen| (conn, screen_num, strlen))
        })
        .and_then(|(conn, screen_num, strlen)| {
            let screen = &conn.setup().roots[screen_num];
            if let Err(e) = x11rb::protocol::xproto::change_property(
                &conn,
                PropMode::Replace,
                screen.root,
                AtomEnum::WM_NAME,
                AtomEnum::STRING,
                8,
                strlen,
                status.as_bytes(),
            )
            .map_err(|_| XSetRoot::PaintingError)
            {
                return Err(e);
            }
            Ok(conn)
        })
        .and_then(|conn| conn.sync().map(|_| ()).map_err(|_| XSetRoot::ConSyncError))
    {
        Ok(()) => (),
        Err(err) => eprintln!("{:?}", err),
    }
}

fn output(status: String, output_format: OutputFormat) {
    match output_format {
        OutputFormat::Newline => println!("{}", status),
        OutputFormat::XSetRoot => xsetroot(status),
    }
}

pub async fn create_server(
    input_data: &mut InputData,
    running: Arc<AtomicBool>,
) -> Result<(), ServerSetup> {
    let mut listener = tokio::net::UnixListener::bind(&input_data.socket_addr)
        .map_err(|_| ServerSetup::SocketConnection)?;

    output(input_data.cache.status(), input_data.output_format);

    while let Some(stream) = listener.next().await {
        let should_listen = running.load(Ordering::SeqCst);
        if !should_listen {
            break;
        }
        match stream {
            Ok(mut stream) => match parse_stream(&mut stream).await {
                Ok(bar_item) => {
                    input_data.cache.update(bar_item.key, bar_item.value);
                    output(input_data.cache.status(), input_data.output_format);
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
