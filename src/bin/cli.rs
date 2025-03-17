use clap::Parser;
use sniffer::capture;
use sniffer::cli;

fn main() {
    let _cli = cli::parse();
    // capture::start(&cli.interface, cli.protocol, cli.port);
}
