use anyhow::{Ok, Result};
use clap::{arg, command, Parser};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RepoContent {
    path: String,
}

#[derive(Debug, Deserialize)]
struct Tree {
    tree: Vec<RepoContent>,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    plugin_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    // remove newline

    let client = reqwest::Client::builder()
        // Set agent to chrome
        .user_agent("Chrome")
        .build()?;
    let response = client
        .get("https://api.github.com/repos/AstroNvim/astrocommunity/git/trees/HEAD?recursive=1")
        .send()
        .await?;
    let json: Tree = response.json().await?;
    // Loop through the tree, and see if any of the paths contains the plugin name
    let matches = json
        .tree
        .iter()
        .filter(|item| item.path.contains(&args.plugin_name))
        .collect::<Vec<&RepoContent>>();
    // Multiple matches, we really only need theone where the path ends with the plugin name
    let match_ = matches
        .iter()
        .filter(|item| item.path.ends_with(&args.plugin_name))
        .collect::<Vec<&&RepoContent>>();
    // Make sure we only have one match
    if match_.len() > 1 {
        println!("Multiple matches found. Please be more specific");
        return Ok(());
fn wait_for_key(required_key: char) -> i32 {
    let mut buffer = [0u8; 1];
    io::stdin().read_exact(&mut buffer).unwrap();

    let pressed_key = buffer[0] as char;

    if pressed_key == required_key {
        0
    } else {
        // Kill the process
        std::process::exit(1);
    }
}
    Ok(())
}
