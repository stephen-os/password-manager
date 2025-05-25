use crate::app::vault;

use egui::{Color32, Frame, Label, RichText, ScrollArea, Sense, Stroke, Ui, Vec2};

pub fn draw_login_screen(ui: &mut Ui, app: &mut vault::Vault) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading(RichText::new("Vaults").size(32.0).strong());
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(16.0);
        ui.label(
            RichText::new("Select a vault to unlock")
                .size(18.0)
                .color(Color32::from_rgb(100, 100, 100)),
        );
        ui.add_space(16.0);
    });

    // Wrap this section in a scrollable area
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            ui.vertical(|ui| {
                draw_vault_cards(ui, app);

                ui.add_space(16.0);

                let frame = Frame::new()
                    .fill(Color32::from_rgb(42, 42, 42))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(255, 165, 0)))
                    .inner_margin(12.0)
                    .outer_margin(0.0);

                let add_response = frame
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.vertical_centered(|ui| {
                            ui.add_space(6.0);
                            ui.add(Label::new(
                                RichText::new("+ Add New Vault")
                                    .size(16.0)
                                    .color(Color32::from_rgb(170, 170, 170)),
                            ));
                            ui.add_space(6.0);
                        });
                    })
                    .response
                    .interact(Sense::click())
                    .on_hover_cursor(egui::CursorIcon::PointingHand);

                if add_response.clicked() {
                    app.popup = vault::Popup::Create;
                }
            });
        });
}

// Function to create the vault card UI
fn draw_vault_cards(ui: &mut Ui, app: &mut vault::Vault) {
    // Card frame styling

    for vault_data in &app.users {
        ui.add_space(8.0);
        let frame = Frame::new()
            .fill(Color32::from_rgb(42, 42, 42))
            .stroke(Stroke::new(1.0, Color32::from_rgb(255, 165, 0)))
            .inner_margin(12.0)
            .outer_margin(0.0);

        let response = frame
            .show(ui, |ui| {
                // Use available width
                ui.set_width(ui.available_width());

                ui.horizontal(|ui| {
                    // Icon placeholder
                    let icon_size = Vec2::new(40.0, 40.0);
                    ui.add_sized(
                        icon_size,
                        Label::new(
                            RichText::new("🔒")
                                .size(24.0)
                                .color(Color32::from_rgb(170, 170, 170)),
                        ),
                    );

                    ui.vertical(|ui| {
                        // Store if we're being hovered in a local variable to use for all text elements
                        let is_hovered = ui.rect_contains_pointer(ui.min_rect());

                        // Text color changes from light gray to white on hover
                        let text_color = if is_hovered {
                            Color32::from_rgb(255, 255, 255)
                        } else {
                            Color32::from_rgb(170, 170, 170)
                        };

                        ui.add(Label::new(
                            RichText::new(&vault_data.name)
                                .size(18.0)
                                .strong()
                                .color(text_color),
                        ));
                        ui.add(Label::new(
                            RichText::new(format!("Last accessed: {}", vault_data.last_accessed))
                                .size(14.0)
                                .color(text_color),
                        ));
                    });
                });
            })
            .response;

        // Add slight size increase on hover (scale effect)
        let hover_response = response
            .interact(Sense::click())
            .on_hover_cursor(egui::CursorIcon::PointingHand);

        // Handle click event
        if hover_response.clicked() {
            app.login_error = None;
            app.popup = vault::Popup::Login;

            // Set the selected user
            app.vault_user = Some(vault_data.clone());
            app.vault_key.clear();
        }
    }
}
