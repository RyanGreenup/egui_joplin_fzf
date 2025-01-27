use crate::bm25::bm25_trigram;
use crate::list::SelectableList;
use crate::note::Note;
use eframe::egui;
use rand::thread_rng;

const FILTER_ID: &str = "title_filter_id";
const BODY_FILTER_ID: &str = "body_filter_id";
const LIST_ID: &str = "notes_list_id";

pub fn run(database: String) -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(MyApp::new(database)))
        }),
    )
}

struct MyApp {
    title_filter: String,
    body_filter: String,
    initialization: bool,
    list: SelectableList,
    database: String,
}

impl MyApp {
    fn new(database: String) -> Self {
        let _rng = thread_rng();
        Self {
            title_filter: "".to_owned(),
            body_filter: String::new(),
            list: SelectableList::new(
                Note::load_all(&database).expect("unable to load database")
            ),
            initialization: true,
            database,
        }
    }
}

impl MyApp {
    fn update_filtered_notes(&mut self) {
        // Get base set of notes
        let mut sorted_notes = if self.body_filter.is_empty() {
            // If body filter is empty, load all notes from database
            Note::load_all(&self.database).unwrap_or_else(|_| Vec::new())
        } else {
            // If we have a body filter, use FTS search
            Note::search(&self.database, &self.body_filter).unwrap_or_else(|_| Vec::new())
        };

        // Then apply title filter if present
        if !self.title_filter.is_empty() {
            let titles: Vec<String> = sorted_notes.iter()
                .map(|note| note.title.clone())
                .collect();

            let sorted_titles = bm25_trigram(&titles, &self.title_filter);

            // Reorder notes based on sorted titles
            let mut title_sorted_notes = Vec::new();
            for title in sorted_titles {
                if let Some(note) = sorted_notes.iter().find(|note| note.title == title) {
                    title_sorted_notes.push(note.clone());
                }
            }
            sorted_notes = title_sorted_notes;
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
        if ctx.input(|i| i.key_pressed(egui::Key::L) && i.modifiers.ctrl) {
            ctx.memory_mut(|mem| mem.request_focus(egui::Id::new(LIST_ID)));
        }
        if ctx.input(|i| i.key_pressed(egui::Key::C) && i.modifiers.ctrl) {
            self.list.copy_selected_to_clipboard(ctx);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.list.print_selected();
            // Automatically close
            std::process::exit(0);

        }
        if ctx.input(|i| i.key_pressed(egui::Key::N) && i.modifiers.ctrl) {
            self.list.move_selection(crate::list::Direction::Down);
        }
        if ctx.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl) {
            self.list.move_selection(crate::list::Direction::Up);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Link Creator");
            ui.horizontal(|ui| {
                let _title_filter = ui.label("Title Filter: ");
                let edit = ui.add(
                    egui::TextEdit::singleline(&mut self.title_filter)
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
                let _body_filter = ui.label("Body Filter: ");
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
            self.list.show(ctx, ui, LIST_ID);
        });
    }
}
