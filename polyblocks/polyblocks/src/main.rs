extern crate tokio;
mod lib;

use lib::blocks::run_blocks;
use lib::cli::{parse_cli_args, suggest_cli_fix};
use lib::server::{create_server, suggest_server_fix};
use std::process;

#[tokio::main]
async fn main() {
    let (mut server_input, blocks) = parse_cli_args().unwrap_or_else(|err| {
        suggest_cli_fix(err);
        process::exit(1);
    });
    let (result, _) = tokio::join!(create_server(&mut server_input), run_blocks(blocks));
    match result {
        Ok(()) => process::exit(0),
        Err(err) => {
            suggest_server_fix(err, server_input.server_id);
            process::exit(1);
        }
    }
}
