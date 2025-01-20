use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "netrick", about = "A powerful network utility tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Info {
        #[command(subcommand)]
        subcommand: InfoCommands,
    },
}

#[derive(Subcommand)]
pub enum InfoCommands {
    Ports,
    Net,
    Board
}