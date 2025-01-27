use crate::bm25::bm25_trigram;
use crate::list::SelectableList;
use crate::note::Note;
use eframe::egui;
use rand::thread_rng;

const FILTER_ID: &str = "title_filter_id";

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

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::S)) {
            ctx.memory_mut(|mem| mem.request_focus(egui::Id::new(FILTER_ID)));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Link Creator");
            ui.horizontal(|ui| {
                let title_filter = ui.label("Title Filter: ");
                let edit =
                    ui.add(egui::TextEdit::singleline(&mut self.name).id(egui::Id::new(FILTER_ID)));

                if edit.changed() {
                    // Get all titles as strings
                    let titles: Vec<String> = self
                        .list
                        .items
                        .iter()
                        .map(|note| note.title.clone())
                        .collect();

                    // Get sorted titles using bm25_trigram
                    let sorted_titles = bm25_trigram(&titles, &self.name);

                    // Reorder notes based on sorted titles
                    let mut sorted_notes = Vec::new();
                    for title in sorted_titles {
                        if let Some(note) = self.list.items.iter().find(|note| note.title == title)
                        {
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
