use std::{env, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use walkdir::WalkDir;

use crate::{file_system, git_operations::GitOperations};

pub static GITHUB_API_TREE_RECURSIVE: Lazy<String> = Lazy::new(|| {
    let file_system = file_system::FileSystem::new();
    format!(
        "https://api.github.com/repos/AstroNvim/astrocommunity/git/trees/{}?recursive=1",
        file_system.astrocommunity_hash
    )
});

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct PluginInfo {
    pub group: String,
    pub name: String,
}

impl ToString for PluginInfo {
    fn to_string(&self) -> String {
        format!("{} [{}]", self.name, self.group)
    }
}

pub struct Astrocommunity;

/// Core logic for astrocommuntiy
impl Astrocommunity {
    pub fn new() -> Self {
        Self
    }

    pub fn get_plugins(self) -> Result<Vec<PluginInfo>> {
        let re = Regex::new(r".*lua/astrocommunity/")?;

        let astrocommunity_dir = Self::find_astrocommunity_folder()?;
        let folders = WalkDir::new(astrocommunity_dir.join("lua/astrocommunity"))
            .min_depth(2)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_owned().to_str().unwrap().to_string())
            .map(|folder| re.replace(&folder, "").to_string());
        let mut plugins: Vec<PluginInfo> = Vec::new();
        for folder in folders {
            let split = folder.split('/').collect::<Vec<&str>>();
            let plugin = PluginInfo {
                group: split[0].to_string(),
                name: split[1].to_string(),
            };
            plugins.push(plugin);
        }
        Ok(plugins)
    }

    /// Find the astrocommunity folder in the `$NVIM_APPNAME` folder. Default to nvim
    fn find_astronvim_local_folder() -> Result<PathBuf> {
        let appname = env::var("NVIM_APPNAME").unwrap_or("nvim".to_string());
        if cfg!(target_os = "macos") {
            // TODO: Remove unwrap
            Ok(dirs::home_dir()
                .unwrap()
                .join(format!(".local/share/{}", appname)))
        } else {
            Ok(dirs::data_local_dir().unwrap().join("nvim"))
        }
    }

    /// Find the astrocommunity folder in the `$NVIM_APPNAME` folder. Default to nvim if not found
    pub fn find_astrocommunity_folder() -> Result<PathBuf> {
        let astronvim_local_folder = Self::find_astronvim_local_folder()?;
        Ok(astronvim_local_folder.join("lazy/astrocommunity"))
    }

    /// Fallback method when astrocommunity is not found
    /// Fallback to the github api to get the tree
    /// TODO: Implmenent this.
    fn fallback() -> Result<()> {
        let git_ops = GitOperations::new();
        let tree = git_ops.get_astrocommunity_tree()?;
        Ok(())
    }
}
