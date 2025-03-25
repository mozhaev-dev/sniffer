use std::fmt::Display;

use crate::cli::logger::log_warn;

pub struct TcpHeader<'d> {
    header_len: usize,
    source: u16,
    destination: u16,
    raw_data: &'d [u8],
}

impl<'d> TcpHeader<'d> {
    pub fn new(data: &'d [u8]) -> Option<Self> {
        let data_offset_words = (data[12] >> 4) & 0x0F;
        let header_len = data_offset_words * 4;
        let header_len: usize = header_len.into();

        if data.len() < header_len {
            log_warn("Can't read TCP packet");
            return None;
        }

        let source = u16::from_be_bytes([data[0], data[1]]);
        let destination = u16::from_be_bytes([data[2], data[3]]);

        Some(Self {
            header_len,
            source,
            destination,
            raw_data: data,
        })
    }

    pub fn get_payload(&self) -> &'d [u8] {
        &self.raw_data[self.header_len..]
    }
}

impl<'d> Display for TcpHeader<'d> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TCP | Src Port: {} -> Dst Port: {}",
            self.source, self.destination
        )
    }
}
