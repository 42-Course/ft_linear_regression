use eframe::NativeOptions;
mod app;

pub mod components;
pub mod settings;
pub mod utils;

fn main() {
    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "Linear Regression GUI",
        options,
        Box::new(|cc| Box::new(app::App::new(cc))),
    );
}
