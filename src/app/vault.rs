use egui::Theme;

use crate::ui::entries;
use crate::ui::login;
use crate::ui::popup;

use crate::data::entry;
use crate::data::user;

pub enum Screen {
    Login,
    Entry,
}

pub struct Vault {
    // State
    pub screen: Screen,

    // intermediate
    pub show_create_popup: bool,
    pub show_login_popup: bool,

    pub login_error: Option<String>,

    pub new_user_name: String,
    pub new_user_password: String,
    pub creation_error: Option<String>,

    // Data
    pub users: Vec<user::User>,
    pub entries: Vec<entry::Entry>,

    // User
    pub vault_user: Option<user::User>,
    pub vault_key: String,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            // State
            screen: Screen::Login,

            // intermediate
            show_create_popup: false,
            show_login_popup: false,

            login_error: None,

            new_user_name: String::new(),
            new_user_password: String::new(),
            creation_error: None,

            // Data
            users: user::load_users(),
            entries: Vec::new(),

            // Key to open and close vault
            vault_key: String::new(),

            // User
            vault_user: None,
        }
    }
}

impl Vault {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Vault {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_theme(Theme::Dark);
        egui::CentralPanel::default().show(ctx, |ui| match self.screen {
            Screen::Login => {
                login::draw_login_screen(ui, self);

                if self.show_create_popup {
                    popup::show_create_popup(ctx, self);
                }

                if self.show_login_popup {
                    popup::show_login_popup(ctx, self);
                }
            }
            Screen::Entry => {
                entries::draw_entry_screen(ui, self);
            }
        });
    }
}
