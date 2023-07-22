mod astrocommunity;
mod file_system;
mod fzf;
mod git_operations;
mod opts;
mod util;

use anyhow::{Ok, Result};
use std::fmt::Write;

use crate::{
    astrocommunity::Astrocommunity,
    opts::{Cli, Commands},
    util::ctrlc_handler,
};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Cli::get_opts();
    ctrlc_handler()?;
    match &opts.commands {
        Some(command) => match command {
            Commands::New {
                astrocommunity_path,
                group,
                name,
            } => {
                return opts.create_new_plugin(astrocommunity_path, group, name);
            }
        },
        None => {}
    }

    println!("Welcome to the astrocommunity cli. Please select the plugins to install by pressing tab. When you're done, press enter and we'll add it to your config.");
    let astro = Astrocommunity::new();
    let plugins = astro.get_plugins()?;
    let mut fzf = fzf::Fzf::new()?;
    fzf.write_to_stdin(&plugins)?;
    let selected_plugins = fzf.get_selected_plugins(&plugins)?;
    let mut import_statement = String::with_capacity(60 * selected_plugins.len());
    for item in selected_plugins.iter() {
        writeln!(
            import_statement,
            "{{ import = \"astrocommunity.{}.{}\", enable = true }},",
            item.group, item.name
        )?;
    }
    opts.ouput_to_prefered(&import_statement)?;
    opts.unroll_code(&selected_plugins)?;
    Ok(())
}
