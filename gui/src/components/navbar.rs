use crate::app::App;
use eframe::egui;

pub struct Navbar;

impl Navbar {
  /// Render the top navbar
  pub fn render(ui: &mut egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
      if ui.button("Toggle Sidebar").clicked() {
        app.toggle_sidebar();
      }

      if ui.button("Reload Model").clicked() {
        app.reload_model();
      }

      if ui.button("Train Model").clicked() {
        app.train_model();
      }
    });
  }
}
