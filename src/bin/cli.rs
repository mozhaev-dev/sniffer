use sniffer::capture;
use sniffer::cli::{commands::Commands, parse};
use sniffer::interfaces::{get_interface, print_interfaces_list};

fn main() {
    let cli_input = parse();

    match cli_input.command {
        Commands::List => print_interfaces_list(),
        Commands::Listen(args) => {
            let interface = get_interface(&args.interface);

            capture::start(interface, args.protocol, args.port);
        }
    }
}
