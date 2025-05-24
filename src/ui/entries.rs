use crate::app::vault;
use crate::data::entry;

use egui::{Color32, CornerRadius, Frame, Label, RichText, Sense, Stroke, TextEdit, Ui, Vec2};

// Public function to draw the entry screen
pub fn draw_entry_screen(ui: &mut Ui, vault: &mut vault::Vault) {
    ui.vertical_centered(|ui| {
        if ui
            .button(RichText::new("💾 Save").color(Color32::from_rgb(0, 169, 255)))
            .clicked()
        {
            if let Some(user) = &vault.vault_user {
                if let Err(e) =
                    entry::save_entries(&vault.entries, &user.name, vault.vault_key.as_str())
                {
                    eprintln!("Failed to save entries: {}", e);
                }
            }
        }

        ui.add_space(16.0);

        // Logout button
        if ui
            .button(RichText::new("🚪 Logout").color(Color32::from_rgb(255, 100, 100)))
            .clicked()
        {
            vault.entries.clear();
            vault.vault_user = None;
            vault.vault_key.clear();
            vault.screen = vault::Screen::Login;
            return;
        }

        // Title with larger font and spacing
        ui.add_space(20.0);
        ui.heading(
            RichText::new(format!(
                "{}'s Vault",
                vault.vault_user.as_ref().unwrap().name
            ))
            .size(32.0)
            .strong(),
        );
        ui.add_space(8.0);

        // Divider
        ui.separator();
        ui.add_space(16.0);

        // Secondary heading
        ui.label(
            RichText::new("Your stored credentials")
                .size(18.0)
                .color(Color32::from_rgb(170, 170, 170)),
        );
        ui.add_space(16.0);
    });

    // Display each entry as a card
    ui.vertical(|ui| {
        for entry in &mut vault.entries {
            ui.add_space(8.0);
            draw_entry_card(ui, entry);
        }
        ui.add_space(16.0);

        // Add new entry card (styled like the vault cards from login)
        let frame = Frame::new()
            .fill(Color32::from_rgb(42, 42, 42))
            .stroke(Stroke::new(1.0, Color32::from_rgb(0, 169, 255)))
            .corner_radius(CornerRadius::same(8))
            .inner_margin(12.0)
            .outer_margin(0.0);

        let add_response = frame
            .show(ui, |ui| {
                // Use available width
                ui.set_width(ui.available_width());

                ui.vertical_centered(|ui| {
                    ui.add_space(6.0);
                    ui.add(Label::new(
                        RichText::new("+ Add New Entry")
                            .size(16.0)
                            .color(Color32::from_rgb(170, 170, 170)),
                    ));
                    ui.add_space(6.0);
                });
            })
            .response
            .interact(Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand);

        // Handle click event for adding a new entry
        if add_response.clicked() {
            // Add new entry
            vault.entries.push(entry::Entry::default());

            // Save entries to file
            if let Some(user) = &vault.vault_user {
                if let Err(e) =
                    entry::save_entries(&vault.entries, &user.name, vault.vault_key.as_str())
                {
                    eprintln!("Failed to save entries: {}", e);
                }
            }
        }
    });
}

// Function to create the entry card UI
fn draw_entry_card(ui: &mut Ui, entry: &mut entry::Entry) {
    // Card frame styling
    let frame = Frame::new()
        .fill(Color32::from_rgb(42, 42, 42))
        .stroke(Stroke::new(1.0, Color32::from_rgb(0, 169, 255)))
        .corner_radius(CornerRadius::same(8))
        .inner_margin(12.0)
        .outer_margin(0.0);

    frame.show(ui, |ui| {
        // Use available width
        ui.set_width(ui.available_width());

        if entry.edit_mode {
            draw_entry_edit_mode(ui, entry);
        } else {
            draw_entry_view_mode(ui, entry);
        }
    });
}

// Function to display entry in view mode
fn draw_entry_view_mode(ui: &mut Ui, entry: &mut entry::Entry) {
    ui.horizontal(|ui| {
        // Service icon/logo placeholder
        let icon_size = Vec2::new(40.0, 40.0);
        ui.add_sized(
            icon_size,
            Label::new(
                RichText::new(get_service_icon(&entry.service))
                    .size(24.0)
                    .color(Color32::from_rgb(170, 170, 170)),
            ),
        );

        // Entry details
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add(Label::new(
                    RichText::new(&entry.service)
                        .size(18.0)
                        .strong()
                        .color(Color32::from_rgb(255, 255, 255)),
                ));

                // Spacer to push edit button to the right
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .button(RichText::new("✏️ Edit").color(Color32::from_rgb(170, 170, 170)))
                        .clicked()
                    {
                        entry.edit_mode = true;
                    }
                });
            });

            ui.add(Label::new(
                RichText::new(format!("Email: {}", entry.email))
                    .color(Color32::from_rgb(170, 170, 170)),
            ));

            // Password field with masked/revealed option
            ui.horizontal(|ui| {
                ui.add(Label::new(
                    RichText::new(format!(
                        "Password: {}",
                        if entry.show_password {
                            &entry.password
                        } else {
                            "••••••••"
                        }
                    ))
                    .color(Color32::from_rgb(170, 170, 170)),
                ));

                if ui
                    .button(
                        RichText::new(if entry.show_password {
                            "🙈"
                        } else {
                            "👁️"
                        })
                        .color(Color32::from_rgb(170, 170, 170)),
                    )
                    .clicked()
                {
                    entry.show_password = !entry.show_password;
                }
            });

            if !entry.description.is_empty() {
                ui.add(Label::new(
                    RichText::new(format!("Note: {}", entry.description))
                        .color(Color32::from_rgb(170, 170, 170)),
                ));
            }
        });
    });
}

// Function to display entry in edit mode
fn draw_entry_edit_mode(ui: &mut Ui, entry: &mut entry::Entry) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Service:").color(Color32::from_rgb(170, 170, 170)));
            ui.add_sized(
                [ui.available_width(), 24.0],
                TextEdit::singleline(&mut entry.service)
                    .text_color(Color32::from_rgb(255, 255, 255)),
            );
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("Email:").color(Color32::from_rgb(170, 170, 170)));
            ui.add_sized(
                [ui.available_width(), 24.0],
                TextEdit::singleline(&mut entry.email).text_color(Color32::from_rgb(255, 255, 255)),
            );
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("Password:").color(Color32::from_rgb(170, 170, 170)));
            ui.add_sized(
                [ui.available_width(), 24.0],
                TextEdit::singleline(&mut entry.password)
                    .text_color(Color32::from_rgb(255, 255, 255)),
            );
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("Note:").color(Color32::from_rgb(170, 170, 170)));
            ui.add_sized(
                [ui.available_width(), 24.0],
                TextEdit::singleline(&mut entry.description)
                    .text_color(Color32::from_rgb(255, 255, 255)),
            );
        });

        ui.horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .button(RichText::new("Save").color(Color32::from_rgb(0, 169, 255)))
                    .clicked()
                {
                    entry.edit_mode = false;
                }

                if ui
                    .button(RichText::new("Cancel").color(Color32::from_rgb(170, 170, 170)))
                    .clicked()
                {
                    entry.edit_mode = false;
                }
            });
        });
    });
}

// Function to get an icon for the service (placeholder implementation)
fn get_service_icon(service: &str) -> &'static str {
    match service.to_lowercase().as_str() {
        s if s.contains("google") => "🌐",
        s if s.contains("facebook") || s.contains("meta") => "👤",
        s if s.contains("apple") => "🍎",
        s if s.contains("amazon") => "📦",
        s if s.contains("microsoft") => "🪟",
        s if s.contains("twitter") || s.contains("x") => "🐦",
        s if s.contains("github") => "🐙",
        s if s.contains("netflix") => "🎬",
        s if s.contains("spotify") => "🎵",
        _ => "🔑", // Default icon
    }
}
