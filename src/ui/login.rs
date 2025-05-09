use crate::app::vault;
use egui::{Button, Color32, CornerRadius, Frame, Label, RichText, Sense, Stroke, Ui, Vec2};

pub fn draw_login_screen(ui: &mut Ui, app: &mut vault::Vault) {
    ui.vertical_centered(|ui| {
        // Title with larger font and spacing
        ui.add_space(20.0);
        ui.heading(RichText::new("Password Vault").size(32.0).strong());
        ui.add_space(8.0);

        // Divider
        ui.separator();
        ui.add_space(16.0);

        // Secondary heading for vault selection
        ui.label(
            RichText::new("Select a vault to unlock")
                .size(18.0)
                .color(Color32::from_rgb(100, 100, 100)),
        );
        ui.add_space(16.0);
    });

    // Vault data: test data
    let vaults = vec![
        VaultCardData {
            name: "Personal Vault",
            last_accessed: "2023-11-15 14:32",
            path: "/path/to/personal_vault",
        },
        VaultCardData {
            name: "Work Vault",
            last_accessed: "2023-11-13 08:20",
            path: "/path/to/work_vault",
        },
        VaultCardData {
            name: "Family Vault",
            last_accessed: "2023-10-30 11:05",
            path: "/path/to/family_vault",
        },
    ];

    // Display each vault as a card with available width
    ui.vertical(|ui| {
        for vault_data in vaults {
            ui.add_space(8.0);
            draw_vault_card(ui, vault_data);
        }
        ui.add_space(16.0);

        // Add new vault card (replacing the button)
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

        // Handle click event for adding a new vault
        if add_response.clicked() {
            println!("Adding new vault");
        }
    });
}

// Function to create the vault card UI
fn draw_vault_card(ui: &mut Ui, vault_data: VaultCardData) {
    // Card frame styling
    let frame = Frame::new()
        .fill(Color32::from_rgb(42, 42, 42))
        .stroke(Stroke::new(1.0, Color32::from_rgb(0, 169, 255)))
        .corner_radius(CornerRadius::same(8))
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
                        RichText::new(vault_data.name)
                            .size(18.0)
                            .strong()
                            .color(text_color),
                    ));
                    ui.add(Label::new(
                        RichText::new(format!("Last accessed: {}", vault_data.last_accessed))
                            .size(14.0)
                            .color(text_color),
                    ));
                    ui.add(Label::new(
                        RichText::new(format!("Path: {}", vault_data.path))
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

    if hover_response.hovered() {
        // Make the card slightly larger on hover
        ui.ctx().animate_value_with_time(
            ui.id().with("hover_animation"),
            1.02, // Scale factor - 2% larger
            0.1,  // Animation time in seconds
        );
    }

    // Handle click event
    if hover_response.clicked() {
        println!("Opening vault: {}", vault_data.name);
    }
}

// Struct to hold vault data
pub struct VaultCardData {
    pub name: &'static str,
    pub last_accessed: &'static str,
    pub path: &'static str,
}
