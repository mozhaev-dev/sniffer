extern crate pcap;

use pcap::Device;

fn main() {
    // Получаем первый доступный интерфейс
    let device = Device::lookup().expect("Не удалось найти сетевые устройства");
    println!("Используем устройство: {}", device.name);

    // Открываем устройство для захвата пакетов
    let mut cap = device.open().unwrap();

    // Захватываем первые 5 пакетов и выводим их длину
    while let Ok(packet) = cap.next() {
        println!("Захвачен пакет длиной: {}", packet.header.len);
    }
}
