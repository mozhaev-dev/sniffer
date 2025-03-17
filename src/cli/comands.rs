use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum ProtocolFilter {
    All,
    Tcp,
    Udp,
    Icmp,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start listening
    Listen(ListenArgs),

    /// List available interfaces
    List,
}

#[derive(Args)]
struct ListenArgs {
    /// Network interface name
    interface: String,

    #[arg(long, short = 'P', default_value = "all")]
    /// Transport layer protocol
    protocol: ProtocolFilter,

    #[arg(long, short = 'p', default_value_t = 0)]
    /// Port, 0 - all ports
    port: u16,
}
