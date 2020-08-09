use std::env;
use telebar::cli::{get_socket_addr, CliParseError};

#[test]
fn my_test() {
    env::remove_var("XDG_RUNTIME_DIR");
    let socket_err = get_socket_addr("3".to_string()).unwrap_err();
    assert_eq!(socket_err, CliParseError::XdgRuntime);
}
