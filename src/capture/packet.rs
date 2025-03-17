use super::ip::IpHeader;
use super::transport::{TcpHeader, UdpHeader};
use crate::cli::comands::ProtocolFilter;
use pcap::Packet;

#[derive(Debug)]
pub struct EthernetHeader {
    pub destination: [u8; 6],
    pub source: [u8; 6],
    pub ethertype: u16,
}

impl EthernetHeader {
    pub fn from_packet(packet: &Packet) -> Option<Self> {
        if packet.data.len() < 14 {
            return None;
        }

        let destination = [
            packet.data[0],
            packet.data[1],
            packet.data[2],
            packet.data[3],
            packet.data[4],
            packet.data[5],
        ];
        let source = [
            packet.data[6],
            packet.data[7],
            packet.data[8],
            packet.data[9],
            packet.data[10],
            packet.data[11],
        ];
        let ethertype = u16::from_be_bytes([packet.data[12], packet.data[13]]);

        Some(EthernetHeader {
            destination,
            source,
            ethertype,
        })
    }

    pub fn display(&self) {
        println!(
            "Ethernet | Src: {:02X?} -> Dst: {:02X?} | Ethertype: 0x{:04X}",
            self.source, self.destination, self.ethertype
        );
    }
}

pub fn process_packet(packet: &Packet, protocol_filter: &ProtocolFilter, port_filter: u16) {
    if let Some(eth) = EthernetHeader::from_packet(packet) {
        eth.display();

        if eth.ethertype == 0x0800 {
            if let Some(ip) = IpHeader::from_bytes(&packet.data[14..]) {
                // Фильтруем по протоколу
                if !matches!(protocol_filter, ProtocolFilter::All) {
                    match ip.protocol {
                        6 if *protocol_filter != ProtocolFilter::Tcp => return, // TCP
                        17 if *protocol_filter != ProtocolFilter::Udp => return, // UDP
                        1 if *protocol_filter != ProtocolFilter::Icmp => return, // ICMP
                        _ => {}
                    }
                }

                ip.display();

                let ip_header_size = ((packet.data[14] & 0x0F) * 4) as usize;
                let transport_data = &packet.data[14 + ip_header_size..];

                match ip.protocol {
                    6 => {
                        // TCP
                        if let Some(tcp) = TcpHeader::from_bytes(transport_data) {
                            // Фильтруем по порту
                            if port_filter != 0
                                && tcp.source_port != port_filter
                                && tcp.destination_port != port_filter
                            {
                                return;
                            }
                            tcp.display();
                        }
                    }
                    17 => {
                        // UDP
                        if let Some(udp) = UdpHeader::from_bytes(transport_data) {
                            if port_filter != 0
                                && udp.source_port != port_filter
                                && udp.destination_port != port_filter
                            {
                                return;
                            }
                            udp.display();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
