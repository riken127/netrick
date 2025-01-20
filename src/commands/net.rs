use crate::utils::display;
use sysinfo::{NetworkData, Networks};

pub fn run() {
    let mut net = Networks::new_with_refreshed_list();

    display::info("Network Devices:");
    for (interface_name, data) in &net {
        println!(
            "{} - Sent: {} bytes, Received: {} bytes",
            interface_name,
            data.total_transmitted(),
            data.total_received()
        );
    }
}