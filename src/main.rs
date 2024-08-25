mod snippet;
use clap::{Args, Parser, Subcommand, ValueEnum};
use snippet::{Snippet, SnippetFile};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "rr")]
#[command(about = "A snippet CLI", long_about = None, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Init {
        location: String,
    },

    New,
    Exec {
        snippet: Option<String>,
    },
}

fn main() -> std::io::Result<()> {
    let snippet = Snippet::new("echo".to_string(), "test".to_string(), None);
    let snippet_file = SnippetFile {
        snippets: vec![snippet],
    };
    let toml = toml::to_string(&snippet_file).unwrap();
    let mut file = File::create("snippets.toml")?;

    file.write_all(toml.as_bytes())?;
    Ok(())
}
