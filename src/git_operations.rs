use anyhow::Result;
use itertools::Itertools;
use serde::Deserialize;

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
pub(crate) struct PluginInfo {
    pub group: String,
    pub name: String,
    pub fzf_string: String,
}
pub struct GitOperations;
impl GitOperations {
    pub fn new() -> Self {
        Self
    }
    pub(crate) fn get_astrocommunity_tree(&self) -> Result<Vec<PluginInfo>> {
        let tree = std::process::Command::new("curl")
            .arg(GITHUB_API_TREE_RECURSIVE)
            .output()
            .expect("failed to execute process");
        Self::parse_response(tree.stdout)
    }

    pub(crate) fn parse_response(response: Vec<u8>) -> Result<Vec<PluginInfo>> {
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
        .filter(|p| !p.contains(".github") && p != "lua/astrocommunity")
        .collect::<Vec<_>>();

    let unique_plugins = plugin_paths
        .iter()
        .map(|path| 
            // Split the path into a vector of strings
            path.split('/').collect::<Vec<_>>()
        )
        .filter(|p| p.len() >= 2)
        .map(|p| PluginInfo {
            group: p[0].to_string(),
            name: p[1].to_string(),
            fzf_string: format!("{} [{}]", p[1], p[0]),
        })
        .collect::<Vec<_>>();
    Ok(unique_plugins)
}
}



