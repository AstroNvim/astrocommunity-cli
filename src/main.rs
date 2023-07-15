mod fzf;

use std::{
    borrow::Cow,
    io::{self},
};

use anyhow::{Ok, Result};

use itertools::Itertools;
use serde::Deserialize;

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
    fzf_string: String,
}

impl PluginInfo {
    fn text(&self) -> Cow<str> {
        Cow::Owned(format!("{} [{}]", &self.name, &self.group))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to the astrocommunity cli. Please select the plugins to install by pressing tab. When you're done, press enter and we'll add it to your config.");
    select_plugins()
}

fn select_plugins() -> Result<()> {
    let plugins = get_astrocommunity_tree()?;
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
        .collect::<Vec<PluginInfo>>();

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

fn get_astrocommunity_tree() -> Result<Vec<PluginInfo>> {
    let tree = std::process::Command::new("curl")
        .arg(GITHUB_API_TREE_RECURSIVE)
        .output()
        .expect("failed to execute process");
    let plugins = parse_response(tree.stdout)?;
    Ok(plugins)
}

fn parse_response(response: Vec<u8>) -> Result<Vec<PluginInfo>> {
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
        .map(|p| PluginInfo {
            group: p[0].to_string(),
            name: p[1].to_string(),
            fzf_string: format!("{} [{}]", p[1], p[0]),
        })
        .collect::<Vec<_>>();
    Ok(unique_plugins)
}
