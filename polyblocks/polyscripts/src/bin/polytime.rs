extern crate tokio;

use polylib::{render_pbwrite_error, write_polyblocks};
use polyscripts::get_app_context;
use std::process;

#[tokio::main]
async fn main() {
    let context = get_app_context(
        "polytime",
        "Print out nice time information on the status bar.",
    );
    let output = chrono::Local::now().format("%-l:%M %p ").to_string();
    match write_polyblocks(&context.socket_addr, &context.block, &output).await {
        Ok(()) => process::exit(0),
        Err(e) => {
            render_pbwrite_error(e);
            process::exit(1);
        }
    }
}
