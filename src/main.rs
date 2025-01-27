#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fmt::Display;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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
    selected_item: Option<usize>,
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
            selected_item: None,
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
        let items_len = self.items.len();

        // Handle keyboard input
        if let Some(selected_item) = self.selected_item {
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                self.selected_item = Some((selected_item + 1).min(items_len - 1));
            }
            if ctx.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                self.selected_item = Some(selected_item.saturating_sub(1));
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Space)) {
                self.item_open[selected_item] = !self.item_open[selected_item];
            }
        }

        for i in 0..items_len {
            let open = self.item_open[i];
            ui.collapsing(format!("{}", self.items[i]), |ui| {
                if Some(i) == self.selected_item {
                    ui.visuals_mut().selection.bg_fill = egui::Color32::from_gray(196);
                }
                ui.label(format!("Body: {}", self.items[i].body));
            });
            self.item_open[i] = open;
        }
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



