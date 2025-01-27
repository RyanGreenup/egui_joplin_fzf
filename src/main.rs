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
            println!("Listing notes from database: {}", cli.database);
            // TODO: Implement list functionality using cli.database
        }
        Some(cli::Commands::Add { title, content }) => {
            println!("Adding note to database: {}", cli.database);
            println!("Title: {}\nContent: {}", title, content);
            // TODO: Implement add functionality using cli.database
        }
        Some(cli::Commands::Search { query }) => {
            println!("Searching in database: {}", cli.database);
            println!("Query: {}", query);
            // TODO: Implement search functionality using cli.database
        }
        None => {
            // No command provided, run the GUI
            ui::run()?
        }
    }

    Ok(())
}
