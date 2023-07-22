use anyhow::Result;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use once_cell::sync::Lazy;

pub(crate) fn print_with_syntax(s: &str) -> anyhow::Result<()> {
    // Load these once at the start of your program
    bat::PrettyPrinter::new()
        .input_from_bytes(s.as_bytes())
        .language("lua")
        .print()?;
    Ok(())
}

pub(crate) fn copy_to_clipboard(import_statement: &str) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(import_statement.to_string()).unwrap();
    println!("Added to clipboard");
    Ok(())
}

pub fn ctrlc_handler() -> Result<()> {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        dbg!("Exiting");
        std::process::exit(0);
    });
    Ok(())
}
