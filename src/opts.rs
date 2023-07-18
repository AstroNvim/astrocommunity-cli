use std::{env, path::PathBuf};

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::{
    astrocommunity::{self, Astrocommunity},
    util::{copy_to_clipboard, print_with_syntax},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub copy_to_clipboard: bool,

    #[arg(short, long)]
    pub output: bool,

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

    /// Create a new folder in atrocommunity directory
    /// The folder should be:
    pub fn create_new_plugin(&self) {
        let astrocommunity_dir =
            astrocommunity::Astrocommunity::find_astrocommunity_folder().unwrap();

        let appname = env::var("NVIM_APPNAME").unwrap_or("nvim".to_string());

        let args = self.commands.as_ref().unwrap().clone();
        match args {
            Commands::New {
                astrocommunity_path,
                group,
                name,
            } => {
                let new_plugin_dir = PathBuf::from(astrocommunity_path).join(group).join(name);
                std::fs::create_dir_all(&new_plugin_dir).unwrap();
                let new_plugin_file = new_plugin_dir.join("init.lua");
                let new_plugin_readme = new_plugin_dir.join("README.md");
                std::fs::write(new_plugin_file, "").unwrap();
                std::fs::write(new_plugin_readme, "").unwrap();
                println!("Created new plugin at {}", new_plugin_dir.to_str().unwrap());
            }
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new astrocommunity plugin
    New {
        /// The path for your astrocommunity path
        #[arg(required = true)]
        astrocommunity_path: String,
        /// The group name of the plugin. Example: pack, editor-support, etc
        #[arg(required = true)]
        group: String,
        /// The name of the plugin. Could be the name of the pack, or the name of the plugin
        #[arg(required = true)]
        name: String,
    },
}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The group name of the plugin. Example: pack, editor-support, etc
    pub group: String,
    /// The name of the plugin. Could be the name of the pack, or the name of the plugin
    pub name: String,
}
pub fn get_opts() -> Cli {
    Cli::parse()
}
