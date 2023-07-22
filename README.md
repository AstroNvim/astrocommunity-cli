# Astrocommunity CLI
This repository contains the code for `astrocommunity-cli`, a community written CLI to search for and generate the code for your favorite astrocommunity plugins.

## 📦 Setup

Currently, we only support installing trough cargo.

To install, you need to run:
```bash
cargo install git@github.com:Uzaaft/astrocommunity-cli
```

## 🔨 Usage

```sh
Usage: 

Commands:
  new   Create a new astrocommunity plugin
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --copy-to-clipboard  Copy the import statement to the clipboard
  -o, --output             Print the import statement to stdout, without syntax highlighting
  -u, --unroll             Unroll the code for the selected plugins
  -h, --help               Print help
  -V, --version            Print version
```
