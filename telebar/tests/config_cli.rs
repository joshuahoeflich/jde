use std::path::PathBuf;
use telebar::cli::get_config_path;

#[test]
fn get_config_file() {
    let mut cli_path = PathBuf::new();
    cli_path.push(file!());
    cli_path.push("Config.toml");
    let config_path = get_config_path(Some(cli_path.to_str().unwrap())).unwrap();
    assert_eq!(config_path, cli_path);
}
