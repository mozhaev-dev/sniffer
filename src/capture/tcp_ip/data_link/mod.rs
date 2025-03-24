mod ethernet;
mod loopback;

use crate::capture::tcp_ip::network::NetworkProtocol;
use ethernet::EthernetHeader;
use loopback::LoopbackHeader;
use pcap::Linktype;

pub enum DataLinkHeader<'d> {
    Ethernet(EthernetHeader<'d>),
    Loopback(LoopbackHeader<'d>),
}

impl<'d> DataLinkHeader<'d> {
    pub fn from_bytes(data: &'d [u8], link_type: Linktype) -> Option<Self> {
        match link_type {
            Linktype::ETHERNET => Some(Self::Ethernet(EthernetHeader::new(data)?)),
            Linktype::NULL => Some(Self::Loopback(LoopbackHeader::new(data)?)),
            _ => None,
        }
    }

    fn get_payload(&self) -> &'d [u8] {
        match self {
            Self::Ethernet(h) => h.get_payload(),
            Self::Loopback(h) => h.get_payload(),
        }
    }

    fn get_network_protocol(&self) -> Option<NetworkProtocol> {
        match self {
            Self::Ethernet(h) => h.get_network_protocol(),
            Self::Loopback(h) => h.get_network_protocol(),
        }
    }
}

pub struct DataLink<'d> {
    pub header: DataLinkHeader<'d>,
    pub network_protocol: Option<NetworkProtocol>,
    pub payload: &'d [u8],
}

impl<'d> DataLink<'d> {
    pub fn new(data: &'d [u8], link_type: Linktype) -> Option<Self> {
        let data_link_header = DataLinkHeader::from_bytes(data, link_type)?;

        let network_protocol = data_link_header.get_network_protocol();
        let payload = data_link_header.get_payload();

        Some(Self {
            header: data_link_header,
            network_protocol,
            payload,
        })
    }
}
