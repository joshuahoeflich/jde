extern crate tokio;

// use std::process;
// use std::sync::atomic::AtomicBool;
// use std::sync::Arc;
// use telebar_server::cli::parse_cli_args;
// use telebar_server::errors::error_message;
// use telebar_server::server::{create_server, suggest_server_fix};
// use telebar_server::signals::register_signal_handler;

#[tokio::main]
async fn main() {
    println!("Write me!");
    // let cli_args = parse_cli_args().unwrap_or_else(|_| {
    //     error_message(
    //         "CONFIG FILE NOT FOUND",
    //         "Please pass a Config.toml or create a config file in ~/.config/telebar/Config.toml"
    //             .to_string(),
    //     );
    //     process::exit(1);
    // });
    // let socket_addr = get_socket_addr(&cli_args.server_id).unwrap_or_else(|_| {
    //     error_message(
    //         "XDG_RUNTIME_NOT_FOUND",
    //         "Please ensure your system follows the XDG Base directory specification.".to_string(),
    //     );
    //     process::exit(1);
    // });
    // let running = Arc::new(AtomicBool::new(true));
    // register_signal_handler(&socket_addr, &running).unwrap_or_else(|_| {
    //     error_message(
    //         "COULD NOT REGISTER SIGNAL HANDLER",
    //         "We cannot gracefully terminate the server, so we are aborting.".to_string(),
    //     )
    // });
    // match create_server(&socket_addr, running).await {
    //     Ok(()) => process::exit(0),
    //     Err(err) => {
    //         suggest_server_fix(err, &socket_addr);
    //         process::exit(1);
    //     }
    // }
}
