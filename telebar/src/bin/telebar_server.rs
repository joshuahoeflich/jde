extern crate tokio;

use std::process;
use telebar::cli::{parse_cli_args, suggest_cli_fix};
use telebar::server::{create_server, suggest_server_fix};

#[tokio::main]
async fn main() {
    let mut input_data = parse_cli_args().unwrap_or_else(|err| {
        suggest_cli_fix(err);
        process::exit(1);
    });
    match create_server(&mut input_data).await {
        Ok(()) => process::exit(0),
        Err(err) => {
            suggest_server_fix(err, input_data.server_id);
            process::exit(1);
        }
    }
}
