#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt::Display;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Debug, Clone)]
struct Note {
    title: String,
    body: String,
    id: String,
}

impl Note {
    fn random(title: &str, body: &str) -> Self {
        let title = title.into();
        let body = body.into();
        let rng = thread_rng();
        let id: String = rng
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        Self { title, body, id }
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

struct MyApp {
    name: String,
    age: u32,
    initialization: bool,
    list: SelectableList,
}

impl Default for MyApp {
    fn default() -> Self {
        let rng = thread_rng();
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            list: SelectableList::new(
                [
                    Note::random("Title 1", "Body 1"),
                    Note::random("Title 2", "Body 2"),
                    Note::random("Title 3", "Body 3"),
                    Note::random("Title 4", "Body 4"),
                    Note::random("Title 5", "Body 5"),
                ]
                .to_vec(),
            ),
            initialization: true,
        }
    }
}

struct SelectableList {
    items: Vec<Note>,
    selected_item: Option<usize>,
    item_open: Vec<bool>,
}

impl SelectableList {
    fn new(items: Vec<Note>) -> Self {
        let len = items.len();
        Self {
            items,
            selected_item: None,
            item_open: vec![false; len],
        }
    }

    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        egui::SidePanel::right("note_preview")
            .resizable(true)
            .min_width(200.0)
            .show_inside(ui, |ui| {
                if let Some(selected) = self.selected_item {
                    ui.heading(&self.items[selected].title);
                    ui.separator();
                    ui.label(&self.items[selected].body);
                } else {
                    ui.label("Select a note to preview");
                }
            });

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, item) in self.items.iter().enumerate() {
                let response = ui.selectable_value(&mut self.selected_item, Some(i), &item.title);

                if response.clicked() || response.secondary_clicked() || response.has_focus() {
                    ui.scroll_to_cursor(Some(egui::Align::Center));
                }

                if Some(i) == self.selected_item {
                    ui.collapsing("Details", |ui| {
                        ui.label(&item.body);
                    });
                }
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Link Creator");
            ui.horizontal(|ui| {
                let name_label = ui.label("Title Filter: ");
                let edit = ui
                    .text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);

                if self.initialization {
                    edit.request_focus();
                    self.initialization = false;
                }
            });
            ui.separator();
            ui.heading("Items List");
            self.list.show(ctx, ui);
        });
    }
}



