use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "noteapp")]
#[command(about = "A simple note-taking application", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all notes
    List,
    /// Add a new note
    Add {
        /// Note title
        title: String,
        /// Note content
        content: String,
    },
    /// Search notes
    Search {
        /// Search query
        query: String,
    },
}
