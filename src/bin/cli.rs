use sniffer::capture::{self, Protocol};
use sniffer::cli::commands::ProtocolFilter;
use sniffer::cli::{commands::Commands, parse};
use sniffer::interfaces::{get_interface, print_interfaces_list};

fn main() {
    let cli_input = parse();

    match cli_input.command {
        Commands::List => print_interfaces_list(),
        Commands::Listen(args) => {
            let interface = get_interface(&args.interface);
            let protocol = match args.protocol {
                ProtocolFilter::All => Protocol::All,
                ProtocolFilter::Tcp => Protocol::Icmp,
                ProtocolFilter::Udp => Protocol::Udp,
                ProtocolFilter::Icmp => Protocol::Icmp,
            };

            capture::start(interface, protocol, args.port);
        }
    }
}
