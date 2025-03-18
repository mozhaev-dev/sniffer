use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone)]
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
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start listening
    Listen(ListenArgs),

    /// List available interfaces
    List,
}

#[derive(Args)]
pub struct ListenArgs {
    /// Network interface name
    pub interface: String,

    #[arg(long, short = 'P', default_value = "all")]
    /// Transport layer protocol
    pub protocol: ProtocolFilter,

    #[arg(long, short = 'p', default_value_t = 0)]
    /// Port, 0 - all ports
    pub port: u16,
}
