use clap::{App, Arg, ArgMatches};
use std::env;
use std::path::PathBuf;

pub struct InputData {
    pub socket_addr: String,
}

#[derive(Debug, PartialEq)]
pub enum CliParseError {
    Home,
    XdgRuntime,
    ConfigFile,
}

pub fn parse_cli_args() -> Result<InputData, CliParseError> {
    let matches = get_app_matches();
    let socket_addr = get_socket_addr(matches.value_of("id").unwrap_or("0").to_owned())?;
    let config_string = std::fs::read_to_string(get_config_path(matches.value_of("config"))?)
        .map_err(|_| CliParseError::ConfigFile)?;
    println!("{}", config_string);
    Ok(InputData { socket_addr })
}

fn get_app_matches<'a>() -> ArgMatches<'a> {
    App::new("telebar-server")
        .version("1.0")
        .author("Joshua Hoeflich")
        .about("Server for telebar.")
        .arg(
            Arg::with_name("id")
                .short("i")
                .long("id")
                .takes_value(true)
                .help("Id of the server you want to start. Defaults to 0."),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Config file for the status bar. Defaults to the path passed here, $TELEBAR_CONFIG_FILE, or ~/.config/telebar.Config.toml."),
        )
        .get_matches()
}

pub fn get_socket_addr(socket_addr: String) -> Result<String, CliParseError> {
    let xdg_runtime_dir = env::var("XDG_RUNTIME_DIR").map_err(|_| CliParseError::XdgRuntime)?;
    let mut socket_buffer = PathBuf::new();
    socket_buffer.push(xdg_runtime_dir);
    socket_buffer.push(format!("{}_telebar_socket", socket_addr));
    Ok(socket_buffer.to_string_lossy().into_owned())
}

pub fn get_config_path(cli_path: Option<&str>) -> Result<PathBuf, CliParseError> {
    cli_path.map_or_else(
        || {
            env::var("TELEBAR_CONFIG_FILE")
                .map(PathBuf::from)
                .or_else(|_| {
                    let mut home_path = env::var("HOME")
                        .map(PathBuf::from)
                        .map_err(|_| CliParseError::Home)?;
                    home_path.push(".config");
                    home_path.push("telebar");
                    home_path.push("Config.toml");
                    Ok(home_path)
                })
        },
        |path| Ok(PathBuf::from(path)),
    )
}
