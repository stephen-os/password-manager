#![windows_subsystem = "windows"]

use eframe::NativeOptions;
use egui::ViewportBuilder;

mod app;
mod data;
mod ui;
mod utils;

use crate::app::vault;

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([360.0, 640.0])
            .with_min_inner_size([360.0, 640.0])
            .with_max_inner_size([360.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Password Manager",
        native_options,
        Box::new(|cc| Ok(Box::new(vault::Vault::new(cc)))),
    )
}
