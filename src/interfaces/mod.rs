use crate::cli::logger::{log_err_exit, log_info};
use pcap::Device;

pub fn get_interfaces_list() -> Vec<Device> {
    Device::list().unwrap_or_else(|_err| log_err_exit("Error: can't get interfaces list"))
}

pub fn print_interfaces_list() {
    let list: Vec<Device> = get_interfaces_list();
    list.iter().for_each(|device| log_info(&device.name));
}

pub fn get_interface(name: &str) -> Device {
    let list: Vec<Device> = get_interfaces_list();
    list.into_iter()
        .find(|device| device.name == name)
        .unwrap_or_else(|| log_err_exit("Error: can't find interface"))
}
