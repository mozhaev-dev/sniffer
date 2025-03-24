use crate::capture::tcp_ip::network::NetworkProtocol;
use crate::cli::logger::log_warn;
use std::fmt::Display;

const LOOPBACK_HEADER_SIZE: usize = 4;

pub struct LoopbackHeader<'d> {
    protocol_identifier: u32,
    raw_data: &'d [u8],
}

impl<'d> LoopbackHeader<'d> {
    pub fn new(data: &'d [u8]) -> Option<Self> {
        if data.len() < LOOPBACK_HEADER_SIZE {
            log_warn("Can't read loopback packet");
            return None;
        }

        let protocol_identifier = u32::from_be_bytes([data[0], data[1], data[3], data[4]]);

        Some(Self {
            protocol_identifier,
            raw_data: data,
        })
    }

    pub fn get_payload(&self) -> &'d [u8] {
        &self.raw_data[LOOPBACK_HEADER_SIZE..]
    }

    pub fn get_network_protocol(&self) -> Option<NetworkProtocol> {
        match self.protocol_identifier {
            0x00000002 => Some(NetworkProtocol::Ip4),
            0x00000018 => Some(NetworkProtocol::Ip4),
            _ => None,
        }
    }
}

impl<'d> Display for LoopbackHeader<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Loopback | Protocol edentifier: 0x{:08X}",
            self.protocol_identifier
        )
    }
}
