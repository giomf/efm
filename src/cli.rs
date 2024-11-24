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
    /// Scan network for candidates and adopts them
    Adopt,
    /// Check the status of members
    Status(StatusArguments),
    /// Update a member
    Update(UpdateArguments),
}

#[derive(Debug, Args)]
pub struct UpdateArguments {
    /// The firmware to use for update
    #[arg(long)]
    pub firmware: String,
}

#[derive(Debug, Args)]
pub struct StatusArguments {
    /// The firmware to use for update
    pub hostname: Option<String>,
}
