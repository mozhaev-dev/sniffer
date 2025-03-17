#[derive(Debug)]
pub struct TcpHeader {
    pub source_port: u16,
    pub destination_port: u16,
}

impl TcpHeader {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 20 {
            return None; // TCP-заголовок минимум 20 байт
        }

        let source_port = u16::from_be_bytes([data[0], data[1]]);
        let destination_port = u16::from_be_bytes([data[2], data[3]]);

        Some(TcpHeader {
            source_port,
            destination_port,
        })
    }

    pub fn display(&self) {
        println!(
            "TCP | Src Port: {} -> Dst Port: {}",
            self.source_port, self.destination_port
        );
    }
}

#[derive(Debug)]
pub struct UdpHeader {
    pub source_port: u16,
    pub destination_port: u16,
}

impl UdpHeader {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 8 {
            return None; // UDP header 8bit min
        }

        let source_port = u16::from_be_bytes([data[0], data[1]]);
        let destination_port = u16::from_be_bytes([data[2], data[3]]);

        Some(UdpHeader {
            source_port,
            destination_port,
        })
    }

    pub fn display(&self) {
        println!(
            "UDP | Src Port: {} -> Dst Port: {}",
            self.source_port, self.destination_port
        );
    }
}
