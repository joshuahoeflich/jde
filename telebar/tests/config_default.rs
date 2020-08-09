use std::env;
use std::path::PathBuf;
use telebar::cli::get_config_path;

#[test]
fn get_config_file() {
    let mut cli_path = PathBuf::new();
    cli_path.push(env::var("HOME").unwrap());
    cli_path.push(".config");
    cli_path.push("telebar");
    cli_path.push("Config.toml");
    let config_path = get_config_path(None).unwrap();
    assert_eq!(config_path, cli_path);
}
