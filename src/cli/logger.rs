use colored::*;
use std::process;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn colorize_text(message: &str, level: LogLevel) -> String {
    match level {
        LogLevel::Info => format!("{}", message.blue().bold()),
        LogLevel::Warning => format!("{}", message.yellow().bold()),
        LogLevel::Error => format!("{}", message.red().bold()),
    }
}

pub fn log(message: &str, level: LogLevel) {
    println!("{}", colorize_text(message, level));
}

pub fn log_info(message: &str) {
    log(message, LogLevel::Info);
}

pub fn log_warn(message: &str) {
    log(message, LogLevel::Warning);
}

pub fn log_err(message: &str) {
    log(message, LogLevel::Error);
}

pub fn log_err_exit(message: &str) -> ! {
    log(message, LogLevel::Error);
    process::exit(1);
}

pub fn log_title(title: &str) {
    println!(
        "{}",
        format!("\n[ {} ]", title.to_uppercase()).cyan().bold()
    );
}
