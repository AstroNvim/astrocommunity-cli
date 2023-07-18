use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::util::{copy_to_clipboard, print_with_syntax};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub copy_to_clipboard: bool,

    #[arg(short, long)]
    pub output: bool,

    #[arg(short, long)]
    pub unroll: bool,

    #[command(subcommand)]
    pub commands: Option<Commands>,
}

impl Cli {
    /// Output the plugins based upon the user provided flags
    pub fn ouput_to_prefered(&self, import_statement: &str) -> Result<()> {
        if self.copy_to_clipboard {
            copy_to_clipboard(import_statement)
        } else if self.output {
            println!("{}", import_statement);
            Ok(())
        } else {
            print_with_syntax(import_statement)
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    // Create a new astrocommunity plugin
    // New(NewArgs),
}

#[derive(Args)]
pub struct NewArgs {
    pub group: Option<String>,
    pub name: Option<String>,
}
pub fn get_opts() -> Cli {
    Cli::parse()
}
