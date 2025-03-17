mod ip;
mod packet;
mod transport;

use crate::cli::commands::ProtocolFilter;
use crate::cli::logger::{log_err_exit, log_info};
use pcap::{Capture, Device};

pub fn start(interface: Device, protocol_filter: ProtocolFilter, port_filter: u16) {
    log_info(&format!(
        "Start capturing on interface: {} (Protocol: {:?}, Port: {})",
        interface.name, protocol_filter, port_filter
    ));

    let mut cap = Capture::from_device(interface)
        .unwrap_or_else(|_err| log_err_exit("Can't open interface"))
        .promisc(true)
        .snaplen(65535)
        .open()
        .unwrap_or_else(|_err| log_err_exit("Can't start capturing"));

    while let Ok(packet) = cap.next() {
        packet::process_packet(&packet, &protocol_filter, port_filter);
    }
}
