use anyhow::anyhow;
use anyhow::Result;
use std::io::prelude::*;
use std::process::{Command, Stdio};

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
            Ok(process) => process,
        };

        Ok(Fzf { process })
    }

    pub fn write_to_stdin(&mut self, input: &[u8]) -> Result<()> {
        match self.process.stdin.as_mut().unwrap().write_all(input) {
            Err(why) => panic!("couldn't write to fzf stdin: {}", why),
            _ => {}
        }
        Ok(())
    }

    pub fn read_from_stdout(&mut self) -> Result<String> {
        let mut s = String::new();
        match self.process.stdout.as_mut().unwrap().read_to_string(&mut s) {
            Err(why) => panic!("couldn't read fzf stdout: {}", why),
            _ => {}
        }
        Ok(s)
    }
}
