use anyhow::{anyhow, Result};
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashSet, process::Command};

static GITHUB_API_TREE_RECURSIVE: &str =
    "https://api.github.com/repos/AstroNvim/astrocommunity/git/trees/HEAD?recursive=1";

const REPO_PATH_PREFIX: &str = "lua/astrocommunity/";

#[derive(Debug, Deserialize)]
struct Tree {
    tree: Vec<RepoContent>,
}

#[derive(Debug, Deserialize)]
struct RepoContent {
    path: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
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
        let output = Command::new("curl")
            .arg(GITHUB_API_TREE_RECURSIVE)
            .output()?;

        if !output.status.success() {
            Err(anyhow!(
                "Curl command failed with exit code {:?}",
                output.status.code()
            ))
        } else {
            Self::parse_response(output.stdout)
        }
    }

    fn parse_response(response: Vec<u8>) -> Result<Vec<PluginInfo>> {
        let tree: Tree = serde_json::from_slice(&response)?;
        let re = Regex::new(r"/[^/]+$")?;

        let mut plugins = HashSet::new();
        for content in tree.tree {
            let path = re.replace(&content.path, "").replace(REPO_PATH_PREFIX, "");
            if !path.contains(".github") && path != REPO_PATH_PREFIX {
                if let Some(plugin) = Self::parse_plugin_info(path) {
                    plugins.insert(plugin);
                }
            }
        }

        Ok(plugins.into_iter().collect())
    }

    fn parse_plugin_info(path: String) -> Option<PluginInfo> {
        let p: Vec<&str> = path.split('/').collect();
        if p.len() >= 2 {
            Some(PluginInfo {
                group: p[0].to_string(),
                name: p[1].to_string(),
                fzf_string: format!("{} [{}]", p[1], p[0]),
            })
        } else {
            None
        }
    }
}
