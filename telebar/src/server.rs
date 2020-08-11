use super::cli::{InputData, OutputFormat};
use super::errors::error_message;
use std::convert::TryFrom;
use tokio::io::AsyncReadExt;
use tokio::stream::StreamExt;

use x11rb::connection::Connection;
use x11rb::protocol::xproto::{AtomEnum, PropMode};
use x11rb::wrapper::ConnectionExt;

pub async fn create_server(input_data: &mut InputData) -> Result<(), std::io::Error> {
    let mut listener = tokio::net::UnixListener::bind(&input_data.socket_addr)?;
    output(input_data.cache.status(), input_data.output_format);

    while let Some(stream) = listener.next().await {
        if let Err(e) = handle_stream(input_data, stream).await {
            eprintln!("{:?}", e);
        }
    }
    Ok(())
}

async fn handle_stream(
    input_data: &mut InputData,
    stream: Result<tokio::net::UnixStream, std::io::Error>,
) -> Result<(), ServerRuntime> {
    if stream.is_err() {
        return Err(ServerRuntime::StreamRead);
    }
    let mut stream = stream.unwrap();
    let mut buffer = Vec::new();
    stream
        .read_to_end(&mut buffer)
        .await
        .map_err(|_| ServerRuntime::StreamRead)?;
    let buffer_string = std::str::from_utf8(&buffer).map_err(|_| ServerRuntime::StringParse)?;
    let bar_item = get_bar_update(buffer_string);
    input_data.cache.update(bar_item.key, bar_item.value);
    output(input_data.cache.status(), input_data.output_format);
    Ok(())
}

fn get_bar_update(buffer_string: &str) -> BarUpdate {
    let mut update = BarUpdate {
        key: String::new(),
        value: String::new(),
    };
    for (counter, line) in buffer_string.lines().enumerate() {
        match counter {
            0 => update.key.push_str(&line.to_lowercase()),
            1 => update.value.push_str(line),
            _ => break,
        }
    }
    update
}

#[derive(Debug)]
enum XSetRoot {
    ConnectionFailed,
    TryFromU32Failure,
    PaintingError,
    ConSyncError,
}

fn output(status: String, output_format: OutputFormat) {
    match output_format {
        OutputFormat::Newline => println!("{}", status),
        OutputFormat::XSetRoot => xsetroot(status),
    }
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

pub fn suggest_server_fix(_: std::io::Error) {
    error_message(
        "COULD NOT CONNECT TO SOCKET",
        "Please make sure you passed a unique id to telebar_server and try again.",
    );
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
