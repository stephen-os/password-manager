mod crypt;
mod entry;
mod ui;
mod users;
mod vault;

use egui::Ui;

use crate::app::entry::Entry;
use crate::app::users::{User, Users};

use rfd::FileDialog;

pub enum Screen {
    Login,
    Entry,
}

pub enum Popup {
    None,
    NewUser,
    Login,
}

pub struct PasswordManager {
    pub users: Users,
    pub entries: Vec<Entry>,
    pub edit_mode: bool,

    // User Creation
    pub new_user: User,
    pub new_user_password: String,

    // Login
    pub login_user: User,
    pub login_password: String,
    pub login_failed: bool,

    // State
    pub screen: Screen,
    pub popup: Popup,
}

impl Default for PasswordManager {
    fn default() -> Self {
        Self {
            users: Users::new(),
            edit_mode: false,
            entries: Vec::new(),

            // New User
            new_user: User::default(),
            new_user_password: String::new(),

            // Login
            login_user: User::default(),
            login_password: String::new(),
            login_failed: false,

            // State
            popup: Popup::None,
            screen: Screen::Login,
        }
    }
}

impl PasswordManager {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    pub fn screen_entry(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.edit_mode { "Done" } else { "Edit" })
            .clicked()
        {
            self.edit_mode = !self.edit_mode;
        }

        if ui.button("Save").clicked() {
            vault::save(
                &self.login_user.vault_path,
                &self.login_user.name,
                &self.login_password,
                &self.entries,
            )
            .expect("Failed to save vault");
        }

        ui.separator();

        ui::draw_entry_grid_header(ui, self.edit_mode);
        ui::draw_entry_grid(ui, &mut self.entries, self.edit_mode);

        if self.edit_mode && ui.button("Add Entry").clicked() {
            self.entries.push(Entry {
                email: String::new(),
                password: String::new(),
                description: String::new(),
            });
        }
    }

    fn screen_login(&mut self, ui: &mut Ui) {
        ui.heading("Login");

        ui.separator();
        ui.label("Select a user to load their vault:");

        for user in &self.users.users {
            if ui.button(&user.name).clicked() {
                self.login_failed = false;
                self.login_user = user.clone();
                self.popup = Popup::Login;
            }
        }

        ui.separator();

        if ui.button("Create New User").clicked() {
            self.popup = Popup::NewUser;
        }

        match self.popup {
            Popup::None => {}
            Popup::NewUser => self.popup_create_new_user(ui),
            Popup::Login => self.popup_login(ui),
        }
    }

    fn popup_create_new_user(&mut self, ui: &mut Ui) {
        egui::Window::new("Create New User")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.new_user.name);

                ui.label("Password:");
                ui.text_edit_singleline(&mut self.new_user_password);

                ui.label("Vault Path:");
                ui.text_edit_singleline(&mut self.new_user.vault_path);

                if ui.button("Browse...").clicked() {
                    if let Some(path) = FileDialog::new()
                        .set_title("Select Vault Location")
                        .pick_folder()
                    {
                        self.new_user.vault_path = path.to_string_lossy().to_string();
                    }
                }

                ui.horizontal(|ui| {
                    if ui.button("Create").clicked() {
                        if !self.new_user.name.is_empty() && !self.new_user.vault_path.is_empty() {
                            // Add user to list
                            self.users.add_user(self.new_user.clone());

                            // Create vault
                            vault::save(
                                &self.new_user.vault_path,
                                &self.new_user.name,
                                &self.new_user_password,
                                &self.entries,
                            )
                            .expect("Failed to save vault");

                            // Clear new user state
                            self.new_user = User::default();
                            self.new_user_password = String::new();

                            // Close popup
                            self.popup = Popup::None;
                        }
                    }

                    if ui.button("Cancel").clicked() {
                        // Clear new user state
                        self.new_user = User::default();
                        self.new_user_password = String::new();

                        // Close popup
                        self.popup = Popup::None;
                    }
                });
            });
    }

    fn popup_login(&mut self, ui: &mut Ui) {
        egui::Window::new(format!("Login: {}", self.login_user.name))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.label("Password:");
                ui.text_edit_singleline(&mut self.login_password);

                if self.login_failed {
                    ui.colored_label(egui::Color32::RED, "Incorrect password.");
                }

                ui.horizontal(|ui| {
                    if ui.button("Login").clicked() {
                        match vault::load(
                            &self.login_user.vault_path,
                            &self.login_user.name,
                            &self.login_password,
                        ) {
                            Ok(entries) => {
                                self.entries = entries;
                                self.screen = Screen::Entry;
                                self.popup = Popup::None;
                            }
                            Err(_) => {
                                self.login_failed = true;
                            }
                        }
                    }

                    if ui.button("Cancel").clicked() {
                        self.login_password.clear();
                        self.login_failed = false;
                        self.popup = Popup::None;
                    }
                });
            });
    }
}

impl eframe::App for PasswordManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| match self.screen {
            Screen::Login => self.screen_login(ui),
            Screen::Entry => self.screen_entry(ui),
        });
    }
}
