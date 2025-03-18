mod ip;
mod packet;
mod transport;

use crate::cli::logger::{log_err_exit, log_info};
use pcap::{Capture, Device};

#[derive(Debug, PartialEq)]
pub enum Protocol {
    All,
    Tcp,
    Udp,
    Icmp,
}

pub fn start(interface: Device, protocol: Protocol, port_filter: u16) {
    log_info(&format!(
        "Start capturing on interface: {} (Protocol: {:?}, Port: {})",
        interface.name, protocol, port_filter
    ));

    let mut cap = Capture::from_device(interface)
        .unwrap_or_else(|_err| log_err_exit("Can't open interface"))
        .promisc(true)
        .snaplen(65535)
        .open()
        .unwrap_or_else(|_err| log_err_exit("Can't start capturing"));

    while let Ok(packet) = cap.next() {
        packet::process_packet(&packet, &protocol, port_filter);
    }
}
