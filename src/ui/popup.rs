use crate::app::vault;
use crate::data::entry;
use crate::data::user;

use chrono::Local;

use egui::{self, Color32, Context};

pub fn show_create_popup(ctx: &Context, app: &mut vault::Vault) {
    egui::Window::new("Create New Vault")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ctx, |ui| {
            ui.label("Vault Name:");
            ui.text_edit_singleline(&mut app.new_user_name);

            ui.label("Password:");
            ui.add(egui::TextEdit::singleline(&mut app.new_user_password).password(true));

            if let Some(error) = &app.creation_error {
                ui.colored_label(Color32::RED, error);
            }

            ui.horizontal(|ui| {
                if ui.button("Create").clicked() {
                    if app.new_user_name.is_empty() || app.new_user_password.is_empty() {
                        app.creation_error = Some("All fields are required.".to_string());
                    } else if app.users.iter().any(|u| u.name == app.new_user_name) {
                        app.creation_error =
                            Some("A vault with this name already exists.".to_string());
                    } else {
                        let new_user = user::User::new(app.new_user_name.clone());

                        app.users.push(new_user);
                        user::save_users(&app.users);
                        entry::save_entries(
                            &Vec::new(),
                            &app.new_user_name,
                            &app.new_user_password,
                        )
                        .unwrap();

                        app.new_user_name.clear();
                        app.new_user_password.clear();
                        app.creation_error = None;
                        app.popup = vault::Popup::None;
                    }
                }

                if ui.button("Cancel").clicked() {
                    app.popup = vault::Popup::None;
                    app.creation_error = None;
                }
            });
        });
}

pub fn show_login_popup(ctx: &Context, app: &mut vault::Vault) {
    if let Some(user) = &app.vault_user {
        let user_name = user.name.clone(); // clone early to avoid borrow conflicts

        egui::Window::new("Unlock Vault")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.label(format!("Vault: {}", user_name));
                ui.label("Enter Password:");
                ui.add(egui::TextEdit::singleline(&mut app.vault_key).password(true));

                if let Some(error) = &app.login_error {
                    ui.colored_label(Color32::RED, error);
                }

                ui.horizontal(|ui| {
                    // Open button
                    if ui.button("Open").clicked() {
                        match crate::data::entry::load_entries(&user_name, &app.vault_key) {
                            Ok(entries) => {
                                app.entries = entries;
                                app.screen = vault::Screen::Entry;
                                app.login_error = None;
                                app.popup = vault::Popup::None;

                                // Update last accessed
                                for user in &mut app.users {
                                    if user.name == user_name {
                                        user.last_accessed =
                                            Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                                        break;
                                    }
                                }
                                user::save_users(&app.users);
                            }
                            Err(e) => {
                                app.login_error = Some(e);
                            }
                        }
                    }

                    // Close button
                    if ui.button("Close").clicked() {
                        app.popup = vault::Popup::None;
                        app.login_error = None;

                        // Clear vault key and selected user
                        app.vault_key.clear();
                        app.vault_user = None;
                    }

                    if ui.button("Delete").clicked() {
                        app.popup = vault::Popup::Delete;
                    }
                });
            });
    } else {
        app.popup = vault::Popup::None;
    }
}

pub fn show_delete_popup(ctx: &Context, app: &mut vault::Vault) {
    if let Some(user) = &app.vault_user {
        let user_name = user.name.clone();

        egui::Window::new("Delete Vault")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.label(format!("Confirm password to delete '{}'", user_name));
                ui.add(egui::TextEdit::singleline(&mut app.vault_key).password(true));

                if let Some(error) = &app.login_error {
                    ui.colored_label(Color32::RED, error);
                }

                ui.horizontal(|ui| {
                    if ui.button("Delete").clicked() {
                        match crate::data::entry::load_entries(&user_name, &app.vault_key) {
                            Ok(_) => {
                                // Remove vault
                                app.users.retain(|u| u.name != user_name);
                                user::save_users(&app.users);

                                // Clear state
                                app.vault_user = None;
                                app.vault_key.clear();
                                app.popup = vault::Popup::None;
                                app.login_error = None;
                            }
                            Err(e) => {
                                app.login_error = Some(e);
                            }
                        }
                    }

                    if ui.button("Cancel").clicked() {
                        app.popup = vault::Popup::None;
                        app.login_error = None;
                        app.vault_user = None;
                        app.vault_key.clear();
                    }
                });
            });
    } else {
        app.popup = vault::Popup::None;
    }
}
