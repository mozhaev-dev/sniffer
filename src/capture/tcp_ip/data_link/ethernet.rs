use crate::capture::tcp_ip::network::NetworkProtocol;
use crate::cli::logger::log_warn;
use std::fmt::Display;

const ETHERNET_HEADER_SIZE: usize = 14;

pub struct EthernetHeader<'d> {
    destination: [u8; 6],
    source: [u8; 6],
    ethertype: u16,
    raw_data: &'d [u8],
}

impl<'d> EthernetHeader<'d> {
    pub fn new(data: &'d [u8]) -> Option<Self> {
        if data.len() < ETHERNET_HEADER_SIZE {
            log_warn("Can't read ethernet packet");
            return None;
        }

        let destination = data[0..6].try_into().unwrap();
        let source = data[6..12].try_into().unwrap();
        let ethertype = u16::from_be_bytes([data[12], data[13]]);

        Some(Self {
            destination,
            source,
            ethertype,
            raw_data: data,
        })
    }

    pub fn get_payload(&self) -> &'d [u8] {
        &self.raw_data[ETHERNET_HEADER_SIZE..]
    }

    pub fn get_network_protocol(&self) -> Option<NetworkProtocol> {
        match self.ethertype {
            0x0800 => Some(NetworkProtocol::Ip4),
            0x86DD => Some(NetworkProtocol::Ip6),
            _ => None,
        }
    }
}

impl<'d> Display for EthernetHeader<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ethernet | Src: {:02X?} -> Dst: {:02X?} | Ethertype: 0x{:04X}",
            self.source, self.destination, self.ethertype
        )
    }
}
