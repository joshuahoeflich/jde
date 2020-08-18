extern crate tokio;

use clap::{App, Arg};
use polylib::{render_pbwrite_error, write_polyblocks};

#[tokio::main]
async fn main() {
    let matches = App::new("pbc")
        .name("Polyblocks Client")
        .version("1.0")
        .author("Joshua Hoeflich")
        .arg(
            Arg::with_name("id")
                .short("i")
                .help("ID of the Polyblocks server you are connecting to. Defaults to 0.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("block")
                .short("b")
                .help("Block in which you would like to display the output.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .help("Output you would like to display in the bar.")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let id = matches.value_of("id").unwrap_or("0");
    let socket_addr = format!("\0{}_polyblocks_socket", id);
    let block = matches.value_of("block").expect("Block is required");
    let output = matches.value_of("output").expect("Output is required");
    match write_polyblocks(&socket_addr, block, output).await {
        Ok(()) => std::process::exit(0),
        Err(err) => {
            render_pbwrite_error(err);
            std::process::exit(1);
        }
    }
}
