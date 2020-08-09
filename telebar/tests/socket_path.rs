extern crate telebar;

use std::env;
use std::path::PathBuf;
use telebar::cli::get_socket_addr;

#[test]
fn my_test() {
    let cwd = env::current_dir().unwrap();
    env::set_var("XDG_RUNTIME_DIR", &cwd);
    let mut socket_buf = PathBuf::new();
    socket_buf.push(cwd);
    socket_buf.push("3_telebar_socket");
    let expected_socket_path = socket_buf.to_string_lossy().into_owned();
    let socket_path = get_socket_addr("3".to_string()).unwrap();
    assert_eq!(socket_path, expected_socket_path);
}
