use egui::{Label, TextEdit, Ui, Vec2};

use crate::app::entry::Entry;

pub fn draw_entry_grid_header(ui: &mut Ui, edit_mode: bool) {
    egui::Grid::new("entry_grid_header")
        .striped(true)
        .show(ui, |ui| {
            ui.label("Email");
            ui.label("Password");
            ui.label("Description");
            if edit_mode {
                ui.label("Actions");
            }
            ui.end_row();
        });
}

pub fn draw_entry_grid(ui: &mut Ui, entries: &mut Vec<Entry>, edit_mode: bool) {
    // Create a mutable variable to store the index of the entry to remove
    let mut to_remove: Option<usize> = None;

    // Set spacing between columns and rows
    ui.spacing_mut().item_spacing.x = 8.0; // horizontal spacing
    ui.spacing_mut().item_spacing.y = 8.0; // vertical spacing

    // Calculate column width
    let total_width = ui.available_width();
    let column_width = if edit_mode {
        (total_width - 80.0) / 3.0 // Reserve space for the button (e.g., 80px)
    } else {
        total_width / 3.0 // No button, equal columns
    };

    // Loop over entries
    for (i, entry) in entries.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.set_height(24.0); // consistent height

            if edit_mode {
                // Draw editable fields
                draw_edit_entry(column_width, 24.0, &mut entry.email, "Email", ui);
                draw_edit_entry(column_width, 24.0, &mut entry.password, "Password", ui);
                draw_edit_entry(column_width, 24.0, &mut entry.description, "Desc", ui);

                // Add remove button at the end
                if ui.button("Remove").clicked() {
                    to_remove = Some(i);
                }
            } else {
                // Draw non-editable fields
                draw_entry(column_width, 24.0, &entry.email, ui);
                draw_entry(column_width, 24.0, &entry.password, ui);
                draw_entry(column_width, 24.0, &entry.description, ui);
            }
        });
    }

    if let Some(index) = to_remove {
        entries.remove(index);
    }
}

fn draw_entry(x: f32, y: f32, content: &String, ui: &mut Ui) {
    ui.add_sized(Vec2::new(x, y), Label::new(content));
}

fn draw_edit_entry(x: f32, y: f32, content: &mut String, hint: &str, ui: &mut Ui) {
    ui.add_sized(
        Vec2::new(x, y),
        TextEdit::singleline(content)
            .hint_text(hint)
            .margin(egui::vec2(4.0, 4.0)),
    );
}
