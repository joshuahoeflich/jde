use super::errors::error_message;
use ctrlc::Error;
use std::os;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn register_signal_handler(socket_addr: &str, running: &Arc<AtomicBool>) -> Result<(), Error> {
    let running = Arc::clone(&running);
    let socket_addr = socket_addr.to_owned();
    ctrlc::set_handler(move || {
        running.store(false, Ordering::SeqCst);
        match os::unix::net::UnixStream::connect(&socket_addr) {
            Ok(_) => {} // Leads to graceful shutdown
            // If we can't connect to the socket here, telebar-server is
            // completely broken and we need to destroy everything
            Err(_) => process::exit(1),
        }
    })
}

pub fn report_register_error(_: Error) {
    error_message(
        "FAILURE TO REGISTER SIGNAL HANDLER",
        "Telebar cannot exit cleanly, so we are aborting now.".to_string(),
    );
}
