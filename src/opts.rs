use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new astrocommunity plugin
    New(NewArgs),
}

#[derive(Args)]
pub struct NewArgs {
    pub group: Option<String>,
    pub name: Option<String>,
}
pub fn get_opts() -> Cli {
    Cli::parse()
}
