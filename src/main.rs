mod fzf;
mod git_operations;

use std::{
    borrow::Cow,
    io::{self},
};

use anyhow::{Ok, Result};

use itertools::Itertools;
use serde::Deserialize;

use cli_clipboard::{ClipboardContext, ClipboardProvider};

use crate::git_operations::GitOperations;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to the astrocommunity cli. Please select the plugins to install by pressing tab. When you're done, press enter and we'll add it to your config.");
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        dbg!("Exiting");
        std::process::exit(0);
    });
    select_plugins()
}

fn select_plugins() -> Result<()> {
    let git_ops = GitOperations::new();
    let plugins = git_ops.get_astrocommunity_tree()?;
    // Convert strings to plugin_name [group_name] format
    let fzf_strings = plugins
        .iter()
        .map(|plugin| plugin.fzf_string.clone())
        // only if not windows
        .join("\n");
    let mut fzf = fzf::Fzf::new()?;
    fzf.write_to_stdin(fzf_strings.as_bytes())?;
    let result = fzf.read_from_stdout()?;
    let selected_plugins = result
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            plugins
                .iter()
                .find(|plugin| plugin.fzf_string == line)
                .unwrap()
                .clone()
        })
        .collect::<Vec<_>>();

    let mut import_statement = String::with_capacity(50 * selected_plugins.len());
    for item in selected_plugins.iter() {
        import_statement.push_str(&format!(
            "{{ import = \"astrocommunity.{group}.{name}\", enable = true }},\n",
            group = item.group,
            name = item.name
        ));
    }
    // Ask the user if they want the import statement to be added to their clipboard
    println!("Do you want to add this to your clipboard? [y/n]");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() == "y" {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(import_statement).unwrap();
        println!("Added to clipboard");
    } else {
        println!("Here's the import statement:");
        println!("{}", import_statement);
    }
    Ok(())
}
