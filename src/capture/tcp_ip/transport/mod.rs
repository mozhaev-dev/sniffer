mod tcp;

use tcp::TcpHeader;

pub enum TransportProtocol {
    All,
    Tcp,
    Udp,
    Icmp,
}

pub enum TransportHeader<'d> {
    Tcp(tcp::TcpHeader<'d>),
    // Udp(udp::UdpHeader<'d>),
    // Icmp(icmp::IcmpHeader<'d>),
}

impl<'d> TransportHeader<'d> {
    pub fn from_bytes(data: &'d [u8], transport_protocol: &TransportProtocol) -> Option<Self> {
        match transport_protocol {
            TransportProtocol::Tcp => Some(Self::Tcp(TcpHeader::new(data)?)),
            // TransportProtocol::Udp => Some(Self::Udp(TcpHeader::new(data)?)),
            // TransportProtocol::Icmp => Some(Self::Icmp(TcpHeader::new(data)?)),
            _ => None,
        }
    }

    fn get_payload(&self) -> &'d [u8] {
        match self {
            Self::Tcp(h) => h.get_payload(),
            // Self::Udp(h) => h.get_payload(),
            // Self::Icmp(h) => h.get_payload(),
        }
    }
}

pub struct Transport<'d> {
    pub header: TransportHeader<'d>,
    pub payload: &'d [u8],
}

impl<'d> Transport<'d> {
    pub fn new(data: &'d [u8], transport_protocol: &TransportProtocol) -> Option<Self> {
        let transport_header = TransportHeader::from_bytes(data, transport_protocol)?;

        let payload = transport_header.get_payload();

        Some(Self {
            header: transport_header,
            payload,
        })
    }
}
