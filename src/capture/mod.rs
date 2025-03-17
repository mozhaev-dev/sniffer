use crate::cli::comands::ProtocolFilter;
use pcap::{Capture, Device};

mod ip;
mod packet;
mod transport;

pub fn start(interface_name: &str, protocol_filter: ProtocolFilter, port_filter: u16) {
    let device = Device::list()
        .expect("Не удалось получить список интерфейсов")
        .into_iter()
        .find(|d| d.name == interface_name);

    let device = match device {
        Some(dev) => dev,
        None => {
            eprintln!("Интерфейс {} не найден!", interface_name);
            return;
        }
    };

    println!(
        "Начинаем захват на интерфейсе: {} (Фильтр: {:?}, Порт: {})",
        device.name, protocol_filter, port_filter
    );

    let mut cap = Capture::from_device(device)
        .expect("Ошибка при открытии устройства")
        .promisc(true)
        .snaplen(65535)
        .open()
        .expect("Не удалось начать захват");

    while let Ok(packet) = cap.next() {
        packet::process_packet(&packet, &protocol_filter, port_filter);
    }
}
