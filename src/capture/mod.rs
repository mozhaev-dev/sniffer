mod tcp_ip;

use crate::cli::logger::{log_err_exit, log_info};
use pcap::{Capture, Device, Linktype, Packet};
use tcp_ip::data_link::DataLink;
use tcp_ip::network::Network;
use tcp_ip::transport::Transport;

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

    let data_link = cap.get_datalink();

    while let Ok(packet) = cap.next() {
        Captured::new(&packet, data_link);
    }
}

pub struct Captured<'d> {
    pub data_link: Option<DataLink<'d>>,
    pub network: Option<Network<'d>>,
    pub transport: Option<Transport<'d>>,
    pub raw_data: &'d [u8],
}

impl<'d> Captured<'d> {
    pub fn new(packet: &'d Packet, link_type: Linktype) -> Option<Self> {
        let data_link = DataLink::new(packet, link_type);

        let network = data_link.as_ref().and_then(|dl| {
            dl.network_protocol
                .as_ref()
                .and_then(|np| Network::new(dl.payload, np))
        });

        let transport = network.as_ref().and_then(|n| {
            n.transport_protocol
                .as_ref()
                .and_then(|tp| Transport::new(n.payload, tp))
        });

        Some(Self {
            data_link,
            network,
            transport,
            raw_data: packet.data,
        })
    }
}
