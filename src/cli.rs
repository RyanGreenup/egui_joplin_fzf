use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "noteapp")]
#[command(about = "A simple note-taking application", long_about = None)]
pub struct Cli {
    /// Path to the notes database
    #[arg(short, long)]
    pub database: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all notes
    List,
    /// Search notes and Print to stdout
    /// TODO Implement body or title
    Search {
        /// Search query
        query: String,
    },
    /// Get the Note Content (useful for fzf)
    Preview {
        id: String,
    }
}
