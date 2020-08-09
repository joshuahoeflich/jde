use std::env;
use telebar::server::get_socket_addr;

#[test]
fn my_test() {
    env::remove_var("XDG_RUNTIME_DIR");
    let socket_err = get_socket_addr("3").unwrap_err();
    assert_eq!(socket_err, env::VarError::NotPresent);
}
