mod cli;
mod commands;
mod utils;

use clap::Parser;
use cli::{Cli, Commands, InfoCommands};
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { subcommand } => match subcommand {
            InfoCommands::Board => commands::board::run(),
            InfoCommands::Net => commands::net::run(),
            InfoCommands::Ports => commands::ports::run(),
        },
    }
}
