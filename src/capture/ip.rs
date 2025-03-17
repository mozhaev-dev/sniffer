#[derive(Debug)]
pub struct IpHeader {
    pub source: [u8; 4],
    pub destination: [u8; 4],
    pub protocol: u8,
}

impl IpHeader {
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 20 {
            return None; // IP-заголовок должен быть минимум 20 байт
        }

        let source = [data[12], data[13], data[14], data[15]];
        let destination = [data[16], data[17], data[18], data[19]];
        let protocol = data[9]; // Протокол (TCP, UDP, ICMP и т. д.)

        Some(IpHeader {
            source,
            destination,
            protocol,
        })
    }

    pub fn display(&self) {
        println!(
            "IP | Src: {}.{}.{}.{} -> Dst: {}.{}.{}.{} | Protocol: {}",
            self.source[0],
            self.source[1],
            self.source[2],
            self.source[3],
            self.destination[0],
            self.destination[1],
            self.destination[2],
            self.destination[3],
            self.protocol
        );
    }
}
