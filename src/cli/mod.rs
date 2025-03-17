pub mod comands;
pub mod logger;

use clap::Parser;
use comands::Cli;

pub fn parse() -> Cli {
    comands::Cli::parse()
}
