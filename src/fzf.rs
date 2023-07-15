use anyhow::Result;
use std::io::prelude::*;
use std::process::{Command, Stdio};

pub struct Fzf {
    process: std::process::Child,
}

impl Fzf {
    pub fn new() -> Result<Self> {
        let process = match Command::new("fzf")
            .arg("-m")
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
