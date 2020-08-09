use std::env;
use telebar::cli::{get_config_path, CliParseError};

#[test]
fn get_config_file() {
    env::remove_var("HOME");
    let config_path = get_config_path(None).unwrap_err();
    assert_eq!(config_path, CliParseError::Home);
}
