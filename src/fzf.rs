use anyhow::{anyhow, Result};
use std::{
    io::prelude::*,
    process::{Command, Stdio},
};

use crate::astrocommunity::PluginInfo;

pub struct Fzf {
    process: std::process::Child,
}

impl Fzf {
    const ERR_FZF_NOT_FOUND: &str = "could not find fzf, is it installed?";
    pub fn new() -> Result<Self> {
        #[cfg(windows)]
        let program = which::which("fzf.exe").map_err(|_| anyhow!(Self::ERR_FZF_NOT_FOUND))?;
        #[cfg(not(windows))]
        let program = which::which("fzf").map_err(|_| anyhow!(Self::ERR_FZF_NOT_FOUND))?;
        match Command::new(program)
            .arg("-m")
            .arg("--height=20")
            .arg("--layout=reverse-list")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Err(why) => panic!("couldn't spawn fzf: {}", why),
            Ok(process) => Ok(Fzf { process }),
        }
    }

    pub fn write_to_stdin(&mut self, input: &[u8]) -> Result<()> {
        if let Err(why) = self.process.stdin.as_mut().unwrap().write_all(input) {
            panic!("couldn't write to fzf stdin: {}", why)
        }
        Ok(())
    }

    pub fn read_from_stdout(&mut self) -> Result<String> {
        let mut s = String::new();
        if let Err(why) = self.process.stdout.as_mut().unwrap().read_to_string(&mut s) {
            panic!("couldn't wait on fzf: {}", why)
        }
        Ok(s)
    }

    pub fn get_selected_plugins(
        &mut self,
        possible_plugins: &[PluginInfo],
    ) -> Result<Vec<PluginInfo>> {
        let selected_plugins = self
            .read_from_stdout()?
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                possible_plugins
                    .iter()
                    .find(|plugin| plugin.to_string() == *line)
                    .unwrap()
                    .clone()
            })
            .collect::<Vec<_>>();
        Ok(selected_plugins)
    }
}
