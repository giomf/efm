use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "ESP Fleet Manager", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Config path
    #[arg(global = true, short, long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands of the application
#[derive(Subcommand, Debug)]
pub enum Commands {
    ///Prints available devices in network
    Adopt,
    Update(Update),
}

#[derive(Debug, Args)]
pub struct Update {
    /// The firmware to use for update
    #[arg(long)]
    pub firmware: String,
}
