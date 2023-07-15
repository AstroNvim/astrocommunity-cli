use std::{
    fs::{read_to_string, File},
    io::{self, Read},
    path::PathBuf,
};

use anyhow::{Ok, Result};

use itertools::Itertools;
use serde::Deserialize;
use skim::prelude::*;

use cli_clipboard::{ClipboardContext, ClipboardProvider};

static GITHUB_API_TREE_RECURSIVE: &str =
    "https://api.github.com/repos/AstroNvim/astrocommunity/git/trees/HEAD?recursive=1";

static GITHUB_API_TREE: &str =
    "https://api.github.com/repos/AstroNvim/astrocommunity/git/trees/HEAD";

#[derive(Debug, Deserialize)]
struct RepoContent {
    path: String,
}

#[derive(Debug, Deserialize)]
struct Tree {
    tree: Vec<RepoContent>,
}

#[derive(Debug, Clone)]
struct PluginInfo {
    group: String,
    name: String,
}

impl SkimItem for PluginInfo {
    fn text(&self) -> Cow<str> {
        Cow::Owned(format!("{} [{}]", &self.name, &self.group))
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(format!("\x1b[3m{}\x1b[m{}", self.name, self.group))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to the astrocommunity cli. Please select the plugins to install by pressing tab. When you're done, press enter and we'll add it to your config.");
    select_plugins()
}

fn select_plugins() -> Result<()> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .no_clear(true)
        .preview(Some("")) // preview should be specified to enable preview window
        .build()?;
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    let plugins = get_astrocommunity_tree()?;
    add_plugins_to_skim(tx_item, plugins);
    let selected_items = Skim::run_with(&options, Some(rx_item))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new)
        .iter()
        .map(|selected_item| {
            (**selected_item)
                .as_any()
                .downcast_ref::<PluginInfo>()
                .unwrap()
                .to_owned()
        })
        .collect::<Vec<PluginInfo>>();

    println!("To install the plugins, add the following to your config:");
    // Create a string with some capacity, to reduce the number of allocations
    // Format of every line
    // {import = "astrocommunity.{group}.{name}", enable = true},
    let mut import_statement = String::with_capacity(50 * selected_items.len());
    for item in selected_items.iter() {
        import_statement.push_str(&format!(
            "{{ import = \"astrocommunity.{group}.{name}\", enable = true }},\n",
            group = item.group,
            name = item.name
        ));
    }
    println!("{}", import_statement);
    // Ask the user if they want the import statement to be added to their clipboard
    println!("Do you want to add this to your clipboard? [y/n]");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() == "y" {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(import_statement).unwrap();
        println!("Added to clipboard");
    }
    Ok(())
}

fn add_plugins_to_skim(tx_item: SkimItemSender, plugins: Vec<(String, String)>) {
    for plugin in plugins {
        tx_item
            .send(Arc::new(PluginInfo {
                group: plugin.0,
                name: plugin.1,
            }))
            .unwrap();
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.
}

fn get_astrocommunity_tree() -> Result<Vec<(String, String)>> {
    // Run this if we are on windows
    let tree = std::process::Command::new("curl")
        .arg(GITHUB_API_TREE_RECURSIVE)
        .output()
        .expect("failed to execute process");
    let plugins = parse_response(tree.stdout)?;
    Ok(plugins)
}

fn parse_response(response: Vec<u8>) -> Result<Vec<(String, String)>> {
    let tree: Tree = serde_json::from_slice(&response)?;
    let re = regex::Regex::new(r"/[^/]+$").unwrap();
    let plugin_paths = tree
        .tree
        .iter()
        .map(|path| {
            re.replace(&path.path, "")
                .replace("lua/astrocommunity/", "")
        })
        .unique()
        // Filtering edge cases
        .filter(|p| !p.contains(".github"))
        .filter(|p| p != "lua/astrocommunity")
        .collect::<Vec<_>>();

    let unique_plugins = plugin_paths
        .iter()
        .map(|path| {
            // Split the path into a vector of strings
            path.split('/').collect::<Vec<_>>()
        })
        .filter(|p| p.len() >= 2)
        .map(|p| (p[0].to_string(), p[1].to_string()))
        .collect::<Vec<_>>();
    Ok(unique_plugins)
}

fn wait_for_key(required_key: char) {
    let mut buffer = [0u8; 1];
    io::stdin().read_exact(&mut buffer).unwrap();

    let pressed_key = buffer[0] as char;

    if pressed_key != required_key {
        std::process::exit(1);
    };
}

async fn listen_to_ctrl_c() {
    tokio::signal::ctrl_c().await.unwrap();

    std::process::exit(0);
}
