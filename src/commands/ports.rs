use std::net::TcpListener;

use crate::utils::display;

pub fn local_port_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn run() {
    display::success("Open Ports:");

    for port in 1024..49151 {
        if !local_port_available(port) {
            display::info(&format!("Port {} is in use", port));
        }
    }
}