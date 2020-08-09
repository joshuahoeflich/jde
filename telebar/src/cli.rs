use super::errors::error_message;
use clap::{App, Arg, ArgMatches};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

type CliResult = Result<InputData, CliParseError>;

pub fn parse_cli_args() -> CliResult {
    let matches = get_app_matches();
    let server_id = matches.value_of("id").unwrap_or("0").to_owned();
    let config_path = matches.value_of("config");
    get_input_data(server_id, config_path)
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
                .help(
                    "ID of the server you want to start. Defaults to 0. You can find the socket at
$XDG_RUNTIME_DIR/${id}_telebar_socket.",
                ),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help(
                    "Config file for the status bar. To find this file, we search three places:

1. The path passed right here,
2. The environment variable $TELEBAR_ENV_VAR_FILE,
3. ~/.config/telebar/Config.toml

If we can't find it in any of this locations, we exit with an error code.",
                ),
        )
        .get_matches()
}

pub fn get_input_data(server_id: String, config_path: Option<&str>) -> CliResult {
    let xdg_runtime = env::var("XDG_RUNTIME_DIR").map_err(|_| CliParseError::XdgRuntime)?;
    let home = env::var("HOME").map_err(|_| CliParseError::Home)?;
    let telebar_env_var = env::var("TELEBAR_ENV_VAR_FILE");
    Ok(InputData {
        socket_addr: get_socket_addr(server_id, xdg_runtime),
        config: Config::new(config_path, home, telebar_env_var)?,
    })
}

pub struct InputData {
    pub socket_addr: String,
    pub config: Config,
}

pub struct Config {
    pub separator: String,
    pub routes: Vec<String>,
    pub values: HashMap<String, String>,
}

type BarResult = Result<String, env::VarError>;

impl Config {
    fn new(
        maybe_cli_str: Option<&str>,
        home_env_var: String,
        telebar_env_var: BarResult,
    ) -> Result<Config, CliParseError> {
        let config_toml = get_config_toml(maybe_cli_str, telebar_env_var, home_env_var)?;
        let config_table = config_toml
            .as_table()
            .ok_or_else(|| CliParseError::TomlParseError)?;

        let mut routes = vec![];
        let mut values = HashMap::new();
        let mut separator: String = "".to_string();
        for key in config_table.keys() {
            if key == "separator" {
                separator.push_str(key);
            } else {
                routes.push(key.to_owned());
                values.insert(key.to_owned(), "NONE".to_owned());
            }
        }

        Ok(Config {
            separator,
            values,
            routes,
        })
    }
}

fn get_socket_addr(socket_addr: String, xdg_runtime: String) -> String {
    let mut socket_buffer = PathBuf::new();
    socket_buffer.push(xdg_runtime);
    socket_buffer.push(format!("{}_telebar_socket", socket_addr));
    socket_buffer.to_string_lossy().into_owned()
}

fn get_config_toml(
    maybe_cli_str: Option<&str>,
    telebar_env_var: BarResult,
    home_env_var: String,
) -> Result<toml::Value, CliParseError> {
    toml::from_str(
        &std::fs::read_to_string(get_config_path(
            maybe_cli_str,
            telebar_env_var,
            home_env_var,
        )?)
        .map_err(|_| CliParseError::ConfigFile)?,
    )
    .map_err(|_| CliParseError::TomlParseError)
}

fn get_config_path(
    maybe_cli_str: Option<&str>,
    telebar_env_var: BarResult,
    home_env_var: String,
) -> Result<PathBuf, CliParseError> {
    maybe_cli_str.map_or_else(
        || {
            telebar_env_var.map(PathBuf::from).or_else(|_| {
                let mut home_path = PathBuf::from(home_env_var);
                home_path.push(".config");
                home_path.push("telebar");
                home_path.push("Config.toml");
                Ok(home_path)
            })
        },
        |path| Ok(PathBuf::from(path)),
    )
}

#[cfg(test)]
mod tests {
    use super::{get_config_path, get_socket_addr};
    use std::path::PathBuf;

    #[test]
    fn socket_addr() {
        assert_eq!(
            get_socket_addr("1".to_string(), "xdg_runtime".to_string()),
            "xdg_runtime/1_telebar_socket"
        );
    }

    #[test]
    fn config_path_specified() {
        let mut cli_path = PathBuf::new();
        cli_path.push(file!());
        cli_path.push("Config.toml");
        let config_path = get_config_path(
            Some(cli_path.to_str().unwrap()),
            Ok("config.toml".to_string()),
            "home".to_string(),
        )
        .unwrap();
        assert_eq!(config_path, cli_path);
    }

    #[test]
    fn config_path_env_var() {
        let config_env_var = PathBuf::from("/path/to/config.toml");
        let config_path = get_config_path(
            None,
            Ok("/path/to/config.toml".to_string()),
            "home".to_string(),
        )
        .unwrap();
        assert_eq!(config_path, config_env_var);
    }

    #[test]
    fn config_path_default() {
        let config_path =
            get_config_path(None, Err(std::env::VarError::NotPresent), "~".to_string()).unwrap();
        assert_eq!(config_path, PathBuf::from("~/.config/telebar/Config.toml"));
    }
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
        CliParseError::Home => error_message(
"$HOME NOT FOUND",
"We cannot find your $HOME directory, so we can't locate your telebar config file. Aborting.".to_string()
),
        CliParseError::XdgRuntime => error_message(
"$XDG_RUNTIME_DIR not found",
"We cannot find the value of $XDG_RUNTIME_DIR, so we can't open a socket. Please make sure your system obeys the XDG base directory specification.".to_string()
),
        CliParseError::ConfigFile => error_message(
"CONFIG FILE ERROR",
"We could not open and parse your configuration file. Try creating one at ~/.config/telebar/Config.toml".to_string()
),
        CliParseError::TomlParseError => error_message(
"TOML PARSE ERROR",
"We could not parse your configuration file into a TOML. Please validate it and try again.".to_string()
),
    }
}
