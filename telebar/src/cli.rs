use super::errors::error_message;
use clap::{App, Arg, ArgMatches};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

type CliResult = Result<InputData, CliParseError>;

pub fn parse_cli_args() -> CliResult {
    let matches = get_app_matches();
    get_input_data(
        matches.value_of("id").unwrap_or("0").to_owned(),
        matches.value_of("config"),
        matches.value_of("output").map_or_else(
            || OutputFormat::Newline,
            |val| {
                if val == "xsetroot" {
                    OutputFormat::XSetRoot
                } else {
                    OutputFormat::Newline
                }
            },
        ),
    )
}

fn get_app_matches<'a>() -> ArgMatches<'a> {
    App::new("telebar")
        .name("telebar")
        .version("1.0")
        .author("Joshua Hoeflich")
        .about("Statusbar server")
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
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help(
                    "Output format for the status bar. Takes two options:

1. \"newlines\" for each update to print a newline, or
2. \"xsetroot\", to write the status changes to the root X window.

If this option is not passed, we default to \"newlines\".",
                ),
        )
        .get_matches()
}

pub fn get_input_data(
    server_id: String,
    config_path: Option<&str>,
    output_format: OutputFormat,
) -> CliResult {
    let xdg_runtime = env::var("XDG_RUNTIME_DIR").map_err(|_| CliParseError::XdgRuntime)?;
    let home = env::var("HOME").map_err(|_| CliParseError::Home)?;
    let telebar_env_var = env::var("TELEBAR_ENV_VAR_FILE");
    Ok(InputData {
        socket_addr: get_socket_addr(server_id, xdg_runtime),
        cache: Cache::new(config_path, home, telebar_env_var)?,
        output_format,
    })
}

#[derive(Clone, Copy)]
pub enum OutputFormat {
    XSetRoot,
    Newline,
}

pub struct InputData {
    pub socket_addr: String,
    pub cache: Cache,
    pub output_format: OutputFormat,
}

pub struct Cache {
    pub separator: String,
    pub routes: Vec<String>,
    pub values: HashMap<String, String>,
}

impl Cache {
    fn new(
        maybe_cli_str: Option<&str>,
        home_env_var: String,
        telebar_env_var: Result<String, env::VarError>,
    ) -> Result<Cache, CliParseError> {
        let config_toml = get_config_toml(maybe_cli_str, telebar_env_var, home_env_var)?;
        let config_table = config_toml
            .as_table()
            .ok_or_else(|| CliParseError::TomlParseError)?;

        let mut routes = vec![];
        let mut values = HashMap::new();
        let mut separator: String = "".to_string();
        for key in config_table.keys() {
            if key == "global" {
                if let Some(toml::Value::String(sep)) = config_table
                    .get(key)
                    .and_then(|table| table.get("separator"))
                {
                    separator.push_str(&sep);
                }
            } else {
                routes.push(key.to_owned());
                values.insert(key.to_owned(), "NONE".to_owned());
            }
        }

        Ok(Cache {
            separator,
            values,
            routes,
        })
    }
    pub fn update(&mut self, key: String, value: String) {
        if !self.values.contains_key(&key) {
            return;
        }
        self.values.insert(key, value);
    }
    pub fn status(&self) -> String {
        let mut output: Vec<String> = vec![];
        for key in &self.routes {
            match self.values.get(key) {
                Some(val) => output.push(val.to_string()),
                None => unreachable!(),
            }
        }
        output.join(&self.separator)
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
    telebar_env_var: Result<String, env::VarError>,
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
    telebar_env_var: Result<String, env::VarError>,
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
