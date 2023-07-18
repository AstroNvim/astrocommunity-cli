use anyhow::Result;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

pub(crate) fn print_with_syntax(s: &str) -> anyhow::Result<()> {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("lua").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in s.lines() {
        let ranges: Vec<(syntect::highlighting::Style, &str)> = h.highlight_line(line, &ps)?;
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        println!("{}", escaped);
    }
    Ok(())
}

pub(crate) fn copy_to_clipboard(import_statement: &str) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(import_statement.to_string()).unwrap();
    println!("Added to clipboard");
    Ok(())
}
