use super::errors::error_message;
use clap::{App, Arg, ArgMatches};
use std::env;
use std::path::PathBuf;

pub fn parse_cli_args() -> Result<InputData, CliParseError> {
    let matches = get_app_matches();

    let xdg_runtime = env::var("XDG_RUNTIME_DIR").map_err(|_| CliParseError::XdgRuntime)?;
    let socket_addr = get_socket_addr(
        matches.value_of("id").unwrap_or("0").to_owned(),
        xdg_runtime,
    );

    let home = env::var("HOME").map_err(|_| CliParseError::Home)?;
    let telebar_config = env::var("TELEBAR_CONFIG_FILE");
    let config = get_config(matches.value_of("config"), home, telebar_config)?;

    Ok(InputData {
        socket_addr,
        config,
    })
}

pub struct InputData {
    pub socket_addr: String,
    pub config: Config,
}

pub struct Config {
    separator: String,
    routes: Vec<String>,
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

fn get_socket_addr(socket_addr: String, xdg_runtime: String) -> String {
    let mut socket_buffer = PathBuf::new();
    socket_buffer.push(xdg_runtime);
    socket_buffer.push(format!("{}_telebar_socket", socket_addr));
    socket_buffer.to_string_lossy().into_owned()
}

type BarResult = Result<String, env::VarError>;

fn get_config(
    maybe_cli_str: Option<&str>,
    home: String,
    telebar_config: BarResult,
) -> Result<Config, CliParseError> {
    let config_table = get_config_toml(maybe_cli_str, home, telebar_config)?
        .as_table()
        .ok_or_else(|| CliParseError::TomlParseError)?;
    Err(CliParseError::Home)
    // Ok(0)
}

fn get_config_toml(
    maybe_cli_str: Option<&str>,
    home: String,
    telebar_config: BarResult,
) -> Result<toml::Value, CliParseError> {
    toml::from_str(
        &std::fs::read_to_string(get_config_path(maybe_cli_str, home, telebar_config)?)
            .map_err(|_| CliParseError::ConfigFile)?,
    )
    .map_err(|_| CliParseError::TomlParseError)
}

fn get_config_path(
    maybe_cli_str: Option<&str>,
    home: String,
    telebar_config: BarResult,
) -> Result<PathBuf, CliParseError> {
    maybe_cli_str.map_or_else(
        || {
            telebar_config
                .and_then(|config| Ok(PathBuf::from(config)))
                .or_else(|_| {
                    let mut home_path = PathBuf::from(home);
                    home_path.push(".config");
                    home_path.push("telebar");
                    home_path.push("Config.toml");
                    Ok(home_path)
                })
        },
        |path| Ok(PathBuf::from(path)),
    )
}

#[derive(Debug, PartialEq)]
pub enum CliParseError {
    Home,
    XdgRuntime,
    ConfigFile,
    TomlParseError,
}

pub fn suggest_cli_fix(err: CliParseError) {
    match err {
        CliParseError::Home => error_message("Something", "useful".to_string()),
        CliParseError::XdgRuntime => error_message("Something", "useful".to_string()),
        CliParseError::ConfigFile => error_message("Something", "useful".to_string()),
        CliParseError::TomlParseError => error_message("Something", "useful".to_string()),
    }
}
