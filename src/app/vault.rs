use egui::Theme;

use crate::ui::entries;
use crate::ui::login;

use crate::data::entry;

pub enum Screen {
    Login,
    Entry,
}

pub struct Vault {
    // State
    pub screen: Screen,

    // Data
    pub entries: Vec<entry::Entry>,
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            entries: entry::get_test_entries(),
            screen: Screen::Login,
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
            }
            Screen::Entry => {
                entries::draw_entry_screen(ui, self);
            }
        });
    }
}
