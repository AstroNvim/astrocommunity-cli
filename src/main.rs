mod astrocommunity;
mod file_system;
mod fzf;
mod git_operations;
mod opts;
mod util;

use anyhow::{Ok, Result};

use crate::{astrocommunity::Astrocommunity, opts::get_opts, util::ctrlc_handler};

#[tokio::main]
async fn main() -> Result<()> {
    ctrlc_handler()?;
    let opts = get_opts();
    println!("Welcome to the astrocommunity cli. Please select the plugins to install by pressing tab. When you're done, press enter and we'll add it to your config.");
    let astro = Astrocommunity::new();
    let plugins = astro.get_plugins()?;
    let mut fzf = fzf::Fzf::new()?;
    fzf.write_to_stdin(&plugins)?;
    let selected_plugins = fzf.get_selected_plugins(&plugins)?;
    let mut import_statement = String::with_capacity(60 * selected_plugins.len());
    for item in selected_plugins.iter() {
        import_statement.push_str(&format!(
            "{{ import = \"astrocommunity.{group}.{name}\", enable = true }},\n",
            group = item.group,
            name = item.name
        ));
    }
    opts.ouput_to_prefered(&import_statement)?;
    Ok(())
}
