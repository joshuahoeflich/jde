use clap::{App, Arg};

pub fn remove_whitespace(mut string: String) -> String {
    string.retain(|c| !c.is_whitespace());
    string
}

#[test]
fn whitespace_test() {
    assert_eq!(
        "Hello!".to_string(),
        remove_whitespace("Hello!\n\n".to_string())
    );
}

pub struct AppContext {
    pub socket_addr: String,
    pub block: String,
    pub command: Option<String>,
}

pub fn get_app_context(app_name: &str, about: &str) -> AppContext {
    let matches = App::new(app_name)
        .name(app_name)
        .version("1.0")
        .author("Joshua Hoeflich")
        .about(about)
        .arg(
            Arg::with_name("id")
                .short("i")
                .long("id")
                .takes_value(true)
                .help("ID of the server to which you would like to connect. Defaults to 0."),
        )
        .arg(
            Arg::with_name("command")
                .short("c")
                .long("command")
                .takes_value(true)
                .help("Command which you would like to pass to this program."),
        )
        .arg(
            Arg::with_name("block")
                .short("b")
                .long("block")
                .required(true)
                .takes_value(true)
                .help("Block which you would like this command to update"),
        )
        .get_matches();
    let id = matches.value_of("id").unwrap_or("0");
    let block = matches
        .value_of("block")
        .expect("Output is required")
        .to_owned();
    let command = matches.value_of("command").map(|command| command.to_owned());
    AppContext {
        socket_addr: get_socket_addr(id),
        block,
        command
    }
}

pub fn get_socket_addr(id: &str) -> String {
    format!("\0{}_polyblocks_socket", id)
}

#[test]
fn socket_addr_test() {
    assert_eq!("\032_polyblocks_socket", get_socket_addr("32"));
}
