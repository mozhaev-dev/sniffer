use std::fmt::Display;

use crate::{capture::tcp_ip::transport::TransportProtocol, cli::logger::log_warn};

pub struct Ip4Header<'d> {
    header_length: usize,
    source: [u8; 4],
    destination: [u8; 4],
    transport_protocol: u8,
    raw_data: &'d [u8],
}

impl<'d> Ip4Header<'d> {
    pub fn new(data: &'d [u8]) -> Option<Self> {
        let ihl = data[0] & 0x0F;
        let header_length = ihl * 4;
        let header_length: usize = header_length.into();

        if data.len() < header_length {
            log_warn("Can't read IP packet");
            return None;
        }

        let source = data[12..16].try_into().unwrap();
        let destination = data[16..20].try_into().unwrap();
        let transport_protocol = data[9];

        Some(Self {
            header_length,
            source,
            destination,
            transport_protocol,
            raw_data: data,
        })
    }

    pub fn get_payload(&self) -> &'d [u8] {
        &self.raw_data[self.header_length..]
    }

    pub fn get_transport_protocol(&self) -> Option<TransportProtocol> {
        match self.transport_protocol {
            6 => Some(TransportProtocol::Tcp),
            17 => Some(TransportProtocol::Udp),
            _ => None,
        }
    }
}

impl<'d> Display for Ip4Header<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IP | Src: {}.{}.{}.{} -> Dst: {}.{}.{}.{} | Protocol: {}",
            self.source[0],
            self.source[1],
            self.source[2],
            self.source[3],
            self.destination[0],
            self.destination[1],
            self.destination[2],
            self.destination[3],
            self.transport_protocol
        )
    }
}
