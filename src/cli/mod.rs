pub mod commands;
pub mod logger;

use clap::Parser;
use commands::Cli;

pub fn parse() -> Cli {
    commands::Cli::parse()
}
