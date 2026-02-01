use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tag")]
#[command(about = "Work journal CLI tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Edit TODO section for today
    Todo,
    /// Add completed item with timestamp
    Done,
    /// Add a note with optional tag
    Note {
        /// Optional tag name (e.g., TIL, memo)
        tag: Option<String>,
    },
}
