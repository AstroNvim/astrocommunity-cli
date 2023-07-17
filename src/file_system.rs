use anyhow::{anyhow, Result};
use dirs::data_local_dir;
use regex::Regex;
use std::{collections::HashSet, fs, path::PathBuf, str::FromStr};
use walkdir::WalkDir;
pub struct FileSystem {
    pub astrocommunity_hash: String,
}

impl FileSystem {
    pub fn new() -> Self {
        let astrocommunity_dir = if cfg!(target_os = "macos") {
            dirs::home_dir().unwrap().join(".local/share/nvim")
        } else {
            data_local_dir().unwrap().join("nvim")
        };
        let lazy_lock = serde_json::from_str::<serde_json::Value>(
            &std::fs::read_to_string(astrocommunity_dir.join("lazy-lock.json")).unwrap(),
        )
        .unwrap();
        let astrocommunity_hash = if lazy_lock["astrocommunity"]["commit"].to_string() != "null" {
            lazy_lock["astrocommunity"]["commit"]
                .to_string()
                .trim_matches('"')
                .to_string()
        } else {
            "HEAD".to_string()
        };
        Self {
            astrocommunity_hash,
        }
    }
}
