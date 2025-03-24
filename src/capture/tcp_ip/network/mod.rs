pub mod ip4;

use super::transport::TransportProtocol;
use ip4::Ip4Header;

pub enum NetworkProtocol {
    Ip4,
    Ip6,
}

pub enum NetworkHeader<'d> {
    Ip4(ip4::Ip4Header<'d>),
    // Ip6(ip6::Ip6Header<'d>) // todo: implement ip6
}

impl<'d> NetworkHeader<'d> {
    pub fn from_bytes(data: &'d [u8], network_protocol: &NetworkProtocol) -> Option<Self> {
        match network_protocol {
            NetworkProtocol::Ip4 => Some(Self::Ip4(Ip4Header::new(data)?)),
            // NetworkProtocol::Ip6 => Some(Self::Ip6(Ip6Header::new(data)?)),
            _ => None,
        }
    }

    fn get_payload(&self) -> &'d [u8] {
        match self {
            Self::Ip4(h) => h.get_payload(),
            // Self::Ip6(h) => h.get_payload(),
        }
    }

    fn get_transport_protocol(&self) -> Option<TransportProtocol> {
        match self {
            Self::Ip4(h) => h.get_transport_protocol(),
            // Self::Ip6(h) => h.get_transport_protocol(),
        }
    }
}

pub struct Network<'d> {
    pub header: NetworkHeader<'d>,
    pub transport_protocol: Option<TransportProtocol>,
    pub payload: &'d [u8],
}

impl<'d> Network<'d> {
    pub fn new(data: &'d [u8], network_protocol: &NetworkProtocol) -> Option<Self> {
        let network_header = NetworkHeader::from_bytes(data, network_protocol)?;

        let transport_protocol = network_header.get_transport_protocol();
        let payload = network_header.get_payload();

        Some(Self {
            header: network_header,
            transport_protocol,
            payload,
        })
    }
}
