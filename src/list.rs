use crate::note::Note;
use eframe::egui;
use egui_demo_lib::easy_mark::easy_mark;
use unindent::unindent;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

pub trait UIMarkdown {
    fn markdown(&mut self, markdown: &str);
}

impl UIMarkdown for egui::Ui {
    fn markdown(&mut self, markdown: &str) {
        easy_mark(self, &unindent(markdown));
    }
}

pub struct SelectableList {
    pub items: Vec<Note>,
    pub selected_item: Option<usize>,
    pub item_open: Vec<bool>,
}

impl SelectableList {
    pub fn new(items: Vec<Note>) -> Self {
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

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, id: &str) {
        // Auto-select first item if nothing is selected
        if self.selected_item.is_none() && !self.items.is_empty() {
            self.selected_item = Some(0);
        }

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
                    // ui.label(&self.items[selected].body);
                    ui.markdown(&self.items[selected].body);
                } else {
                    ui.label("Select a note to preview");
                }
            });

        egui::ScrollArea::vertical()
            .id_source(id)
            .show(ui, |ui| {
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
