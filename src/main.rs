mod fzf;
mod git_operations;
mod opts;
mod util;

use anyhow::{Ok, Result};

use itertools::Itertools;

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use opts::Cli;
use util::print_with_syntax;

use crate::{git_operations::GitOperations, opts::get_opts};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = get_opts();
    println!("Welcome to the astrocommunity cli. Please select the plugins to install by pressing tab. When you're done, press enter and we'll add it to your config.");
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        dbg!("Exiting");
        std::process::exit(0);
    });
    select_plugins(&opts)
}

fn select_plugins(opts: &Cli) -> Result<()> {
    let git_ops = GitOperations::new();
    let plugins = git_ops.get_astrocommunity_tree()?;
    // Convert strings to plugin_name [group_name] format
    let fzf_strings = plugins
        .iter()
        .map(|plugin| plugin.fzf_string.clone())
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
    match &opts.copy_to_clipboard {
        true => copy_to_clipboard(import_statement)?,
        false => match opts.output {
            true => println!("{}", import_statement),
            false => print_with_syntax(&import_statement),
        },
    }
    Ok(())
}

fn copy_to_clipboard(import_statement: String) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(import_statement).unwrap();
    println!("Added to clipboard");
    Ok(())
}
