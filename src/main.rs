use eframe::NativeOptions;

mod app;

fn main() -> Result<(), eframe::Error> {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Password Manager",
        native_options,
        Box::new(|cc| Ok(Box::new(app::PasswordManager::new(cc)))),
    )
}
