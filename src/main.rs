#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod bm25;
mod note;
mod ui;
use ui::run;
mod list;



fn main() -> eframe::Result {
    run()
}

