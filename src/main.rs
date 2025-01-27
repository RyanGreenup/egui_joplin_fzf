#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
mod bm25;
mod note;
use bm25::bm25_trigram;
use note::Note;
use rand::thread_rng;
use egui_demo_lib::easy_mark::easy_mark;
use unindent::unindent;

pub trait UIMarkdown {
    fn markdown(&mut self, markdown: &str);
}

impl UIMarkdown for egui::Ui {
    fn markdown(&mut self, markdown: &str) {
        easy_mark(self, &unindent(markdown));
    }
}

const FILTER_ID: &str = "title_filter_id";

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

struct MyApp {
    name: String,
    age: u32,
    initialization: bool,
    list: SelectableList,
}

impl Default for MyApp {
    fn default() -> Self {
        let _rng = thread_rng();
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            list: SelectableList::new(
                [
                    Note::random("Pythagorean Theorem", r#"# Heading \n content \n In a right triangle, a² + b² = c²"#),
                    Note::random("Golden Ratio", "The golden ratio φ ≈ 1.618033988749895"),
                    Note::random("Euler's Number", "e ≈ 2.718281828459045, base of natural logarithms"),
                    Note::random("Pi", "π ≈ 3.14159265359, ratio of circle's circumference to diameter"),
                    Note::random("Fibonacci Sequence", "0, 1, 1, 2, 3, 5, 8, 13, 21..."),
                ]
                .to_vec(),
            ),
            initialization: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
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

    fn move_selection(&mut self, direction: Direction) {
        match (direction, self.selected_item) {
            (Direction::Down, Some(selected)) if selected < self.items.len() - 1 => {
                self.selected_item = Some(selected + 1);
            }
            (Direction::Down, None) if !self.items.is_empty() => {
                self.selected_item = Some(0);
            }
            (Direction::Up, Some(selected)) if selected > 0 => {
                self.selected_item = Some(selected - 1);
            }
            _ => {}
        }
    }

    fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Handle j/k and arrow keys
        if ctx.input(|i| i.key_pressed(egui::Key::J) || i.key_pressed(egui::Key::ArrowDown)) {
            self.move_selection(Direction::Down);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::K) || i.key_pressed(egui::Key::ArrowUp)) {
            self.move_selection(Direction::Up);
        }

        egui::SidePanel::right("note_preview")
            .resizable(true)
            .min_width(200.0)
            .show_inside(ui, |ui| {
                if let Some(selected) = self.selected_item {
                    ui.heading(&self.items[selected].title);
                    ui.separator();
                    ui.markdown(&self.items[selected].body);
                } else {
                    ui.label("Select a note to preview");
                }
            });

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (i, item) in self.items.iter().enumerate() {
                let response = ui.selectable_value(&mut self.selected_item, Some(i), &item.title);

                // Auto-scroll when selection changes
                if response.clicked()
                    || response.secondary_clicked()
                    || response.has_focus()
                    || (self.selected_item == Some(i) && response.gained_focus())
                {
                    ui.scroll_to_cursor(Some(egui::Align::Center));
                }

                // Show details immediately for selected item
                if Some(i) == self.selected_item {
                    ui.label(&item.body);
                }
            }
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::S)) {
            ctx.memory_mut(|mem| mem.request_focus(egui::Id::new(FILTER_ID)));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Link Creator");
            ui.horizontal(|ui| {
                let title_filter = ui.label("Title Filter: ");
                let edit = ui.add(egui::TextEdit::singleline(&mut self.name)
                    .id(egui::Id::new(FILTER_ID)));

                if edit.changed() {
                    // Get all titles as strings
                    let titles: Vec<String> = self.list.items.iter()
                        .map(|note| note.title.clone())
                        .collect();

                    // Get sorted titles using bm25_trigram
                    let sorted_titles = bm25_trigram(&titles, &self.name);

                    // Reorder notes based on sorted titles
                    let mut sorted_notes = Vec::new();
                    for title in sorted_titles {
                        if let Some(note) = self.list.items.iter()
                            .find(|note| note.title == title) {
                            sorted_notes.push(note.clone());
                        }
                    }

                    // Update the list with sorted notes
                    self.list = SelectableList::new(sorted_notes);
                }

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
