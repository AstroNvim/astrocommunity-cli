use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

use crate::{
    astrocommunity::{Astrocommunity, PluginInfo},
    util::{copy_to_clipboard, print_with_syntax},
};

use std::fmt::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Copy the import statement to the clipboard
    #[arg(short, long)]
    pub copy_to_clipboard: bool,

    /// Print the import statement to stdout, without syntax highlighting
    #[arg(short, long)]
    pub output: bool,

    /// Unroll the code for the selected plugins
    #[arg(short, long)]
    pub unroll: bool,

    #[command(subcommand)]
    pub commands: Option<Commands>,
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

impl Cli {
    /// Parse the command line arguments
    pub fn get_opts() -> Self {
        Self::parse()
    }

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
    pub fn create_new_plugin(&self, path: &str, group: &str, name: &str) -> Result<()> {
        let new_plugin_dir = PathBuf::from(path).join(group).join(name);
        std::fs::create_dir_all(&new_plugin_dir)?;
        let new_plugin_file = new_plugin_dir.join("init.lua");
        let new_plugin_readme = new_plugin_dir.join("README.md");
        std::fs::write(new_plugin_file, "")?;
        std::fs::write(new_plugin_readme, "")?;
        println!("Created new plugin at {}", new_plugin_dir.to_str().unwrap());
        Ok(())
    }

    /// Unroll code
    pub fn unroll_code(&self, plugins: &[PluginInfo]) -> Result<()> {
        if !self.unroll {
            return Ok(());
        }
        let astrocommunity_dir = Astrocommunity::find_astrocommunity_folder()?;
        // Preallocate the string to avoid reallocations
        let mut plugin_code = String::with_capacity(100 * plugins.len());
        for plugin in plugins.iter() {
            let plugin_path = astrocommunity_dir.join(format!(
                "lua/astrocommunity/{}/{}/init.lua",
                plugin.group, plugin.name
            ));

            writeln!(plugin_code, "{}", std::fs::read_to_string(plugin_path)?)?;
        }
        self.ouput_to_prefered(&plugin_code)?;
        Ok(())
    }
}
