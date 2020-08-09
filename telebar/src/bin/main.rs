extern crate tokio;

use std::process;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use telebar::cli::{parse_cli_args, suggest_cli_fix};
use telebar::server::{create_server, suggest_server_fix};
use telebar::signals::{register_signal_handler, report_register_error};
// use telebar::signals::register_signal_handler;

#[tokio::main]
async fn main() {
    let mut input_data = parse_cli_args().unwrap_or_else(|err| {
        suggest_cli_fix(err);
        process::exit(1);
    });
    let running = Arc::new(AtomicBool::new(true));
    register_signal_handler(&input_data.socket_addr, &running).unwrap_or_else(|err| {
        report_register_error(err);
        process::exit(1);
    });
    match create_server(&mut input_data, running).await {
        Ok(()) => process::exit(0),
        Err(err) => {
            suggest_server_fix(err, &input_data.socket_addr);
            process::exit(1);
        }
    }
}
