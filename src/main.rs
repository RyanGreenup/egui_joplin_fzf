#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod bm25;
mod cli;
mod list;
mod note;
mod ui;

use clap::Parser;
use cli::Cli;
use note::Note;

fn main() -> eframe::Result {
    let cli = Cli::parse();

    match cli.command {
        Some(cli::Commands::List) => {
            let mut notes = Note::load_all(&cli.database).expect("Unable to load database");
            notes.reverse();
            for n in notes {
                println!("{}\t {}", n.id, n.title);
            }

            // TODO: Implement list functionality using cli.database
        }
        Some(cli::Commands::Search { query }) => {
            println!("Searching in database: {}", cli.database);
            println!("Query: {}", query);
            // TODO: Implement search functionality using cli.database
            let mut notes =
                Note::search(&cli.database, query.as_str()).expect("Unable to load database");
            notes.reverse();
            for n in notes {
                println!("{}\t {}", n.id, n.title);
            }
        }
        Some(cli::Commands::Preview { id }) => {
            let body = Note::get_body_by_id(&cli.database, id.as_str()).expect("Unable to load database");
            println!("{body}");
        }
        None => {
            // No command provided, run the GUI
            ui::run(cli.database)?
        }
    }

    Ok(())
}
