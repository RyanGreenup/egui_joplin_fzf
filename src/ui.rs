use crate::bm25::bm25_trigram;
use crate::list::SelectableList;
use crate::note::Note;
use eframe::egui;
use rand::thread_rng;

const FILTER_ID: &str = "title_filter_id";
const BODY_FILTER_ID: &str = "body_filter_id";

pub fn run() -> eframe::Result {
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
    body_filter: String,
    age: u32,
    initialization: bool,
    list: SelectableList,
}

impl Default for MyApp {
    fn default() -> Self {
        let _rng = thread_rng();
        Self {
            name: "Arthur".to_owned(),
            body_filter: String::new(),
            age: 42,
            list: SelectableList::new(
                [
                    Note::random(
                        "Pythagorean Theorem",
                        "# Heading \n content \n In a right triangle, a² + b² = c²",
                    ),
                    Note::random("Golden Ratio", "The golden ratio φ ≈ 1.618033988749895"),
                    Note::random(
                        "Euler's Number",
                        "e ≈ 2.718281828459045, base of natural logarithms",
                    ),
                    Note::random(
                        "Pi",
                        "π ≈ 3.14159265359, ratio of circle's circumference to diameter",
                    ),
                    Note::random("Fibonacci Sequence", "0, 1, 1, 2, 3, 5, 8, 13, 21..."),
                ]
                .to_vec(),
            ),
            initialization: true,
        }
    }
}

impl MyApp {
    fn update_filtered_notes(&mut self) {
        // Get all notes
        let mut sorted_notes = self.list.items.clone();
        
        // Filter by title if there's a title filter
        if !self.name.is_empty() {
            let titles: Vec<String> = sorted_notes.iter()
                .map(|note| note.title.clone())
                .collect();
            
            let sorted_titles = bm25_trigram(&titles, &self.name);
            
            // Reorder notes based on sorted titles
            let mut title_sorted_notes = Vec::new();
            for title in sorted_titles {
                if let Some(note) = sorted_notes.iter().find(|note| note.title == title) {
                    title_sorted_notes.push(note.clone());
                }
            }
            sorted_notes = title_sorted_notes;
        }
        
        // Filter by body if there's a body filter
        if !self.body_filter.is_empty() {
            let bodies: Vec<String> = sorted_notes.iter()
                .map(|note| note.body.clone())
                .collect();
            
            let sorted_bodies = bm25_trigram(&bodies, &self.body_filter);
            
            // Reorder notes based on sorted bodies
            let mut body_sorted_notes = Vec::new();
            for body in sorted_bodies {
                if let Some(note) = sorted_notes.iter().find(|note| note.body == body) {
                    body_sorted_notes.push(note.clone());
                }
            }
            sorted_notes = body_sorted_notes;
        }
        
        // Update the list with filtered notes
        self.list = SelectableList::new(sorted_notes);
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            ctx.memory_mut(|mem| mem.request_focus(egui::Id::new(FILTER_ID)));
        }
        if ctx.input(|i| i.key_pressed(egui::Key::B) && i.modifiers.ctrl) {
            ctx.memory_mut(|mem| mem.request_focus(egui::Id::new(BODY_FILTER_ID)));
        }
        // Map Ctrl+L to focus the list AI!

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Link Creator");
            ui.horizontal(|ui| {
                let title_filter = ui.label("Title Filter: ");
                let edit = ui.add(
                    egui::TextEdit::singleline(&mut self.name)
                        .id(egui::Id::new(FILTER_ID))
                );

                if edit.changed() {
                    self.update_filtered_notes();
                }

                if self.initialization {
                    edit.request_focus();
                    self.initialization = false;
                }
            });
            ui.horizontal(|ui| {
                let body_filter = ui.label("Body Filter: ");
                let body_edit = ui.add(
                    egui::TextEdit::singleline(&mut self.body_filter)
                        .id(egui::Id::new(BODY_FILTER_ID))
                );

                if body_edit.changed() {
                    self.update_filtered_notes();
                }
            });

            ui.separator();
            ui.heading("Items List");
            self.list.show(ctx, ui);
        });
    }
}
