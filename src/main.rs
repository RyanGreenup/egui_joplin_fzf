#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod bm25;
mod note;
mod ui;
mod list;
mod cli;

use clap::Parser;
use cli::Cli;

fn main() -> eframe::Result {
    let cli = Cli::parse();

    match cli.command {
        Some(cli::Commands::List) => {
            println!("Listing notes...");
            // TODO: Implement list functionality
        }
        Some(cli::Commands::Add { title, content }) => {
            println!("Adding note: {} - {}", title, content);
            // TODO: Implement add functionality
        }
        Some(cli::Commands::Search { query }) => {
            println!("Searching for: {}", query);
            // TODO: Implement search functionality
        }
        None => {
            // No command provided, run the GUI
            ui::run()?
        }
    }

    Ok(())
}
