use pnet::datalink::{self, NetworkInterface};

pub fn run() {
    // Get all network interfaces
    let interfaces = datalink::interfaces();

    // Filter and display active non-loopback interfaces
    for interface in interfaces {
        if interface.is_up() && interface.is_broadcast() && !interface.is_loopback() {
            println!(
                "Name: {}\n  MAC Address: {}\n",
                interface.name,
                interface.mac.map_or("N/A".to_string(), |mac| mac.to_string())
            );

            // Print all IP addresses assigned to this interface
            for ip in interface.ips {
                println!("  IP: {}", ip);
            }
            println!();
        }
    }
}
