use clap::{App, Arg, ArgMatches};
use std::env;
use std::path::PathBuf;

pub struct InputData {
    pub socket_addr: String,
    pub config: String,
}

#[derive(Debug, PartialEq)]
pub enum CliParseError {
    Home,
    XdgRuntime,
    ConfigFile,
    TomlParseError,
}

pub fn parse_cli_args() -> Result<InputData, CliParseError> {
    let matches = get_app_matches();
    let socket_addr = get_socket_addr(matches.value_of("id").unwrap_or("0").to_owned())?;

    let config = std::fs::read_to_string(get_config_path(matches.value_of("config"))?)
        .map_err(|_| CliParseError::ConfigFile)?;
    let toml: toml::Value = toml::from_str(&config).map_err(|_| CliParseError::TomlParseError)?;
    for i in toml.as_table().unwrap().keys() {
        println!("i is {}", i);
    }
    // println!("{:?}", toml.as_table());
    // for i in toml.as_array() {
    //     println!("I RAN");
    //     for j in i {
    //         println!("THING IS {}", j);
    //     }
    // }
    println!("THIS RAN");
    Ok(InputData {
        socket_addr,
        config,
    })
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

pub fn get_config_path(maybe_cli_str: Option<&str>) -> Result<PathBuf, CliParseError> {
    maybe_cli_str.map_or_else(
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

pub fn parse_config_table(maybe_cli_str: Option<&str>) -> Result<Vec<String>, CliParseError> {
    let toml_raw: toml::Value = toml::from_str(
        &std::fs::read_to_string(get_config_path(maybe_cli_str)?)
            .map_err(|_| CliParseError::ConfigFile)?,
    )
    .map_err(|_| CliParseError::TomlParseError)?;
    toml_raw
        .as_table()
        .ok_or_else(|| CliParseError::TomlParseError)?;
    Ok(vec![])
}
