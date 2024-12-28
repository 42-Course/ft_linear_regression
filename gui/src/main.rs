use eframe::NativeOptions;
mod app;

pub mod components;

fn main() {
    let options = NativeOptions::default();
    let _ = eframe::run_native(
        "Linear Regression GUI",
        options,
        Box::new(|_cc| Ok(Box::new(app::App::default()))),
    );
}
