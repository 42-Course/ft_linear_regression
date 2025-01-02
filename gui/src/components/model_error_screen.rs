use log::info;
use crate::app::App;

pub struct ModelErrorScreen;

impl ModelErrorScreen {
  /// Render model Error Screen
  pub fn render(ui: &mut eframe::egui::Ui, app: &mut App) {
    ui.vertical_centered(|ui| {
      ui.label(eframe::egui::RichText::new("🚧 Oops! Something went wrong 🚧")
        .heading()
        .color(eframe::egui::Color32::RED));
      ui.add_space(ui.available_height() * 0.3);
      ui.add_space(20.0);
      ui.label("It seems like the regression model is not properly initialized.");
      if let Some(error) = &app.error_message {
        ui.add_space(10.0);
        ui.label(eframe::egui::RichText::new(error)
          .color(eframe::egui::Color32::LIGHT_RED)
          .monospace());
      }
      ui.add_space(20.0);
      ui.heading("Here are a few things you can try:");
      ui.label("Ensure the data is setup correctly in the `.env`.");
      ui.label("Ensure the dataset is loaded correctly.");
      ui.label("Check the logs for detailed error messages.");
      ui.add_space(20.0);

      if ui.button("🔄 Retry Initialization").clicked() {
        info!("Retrying model initialization...");
        app.reload_model();
      }

      ui.add_space(20.0);
      ui.label("Or just enjoy this cool animation while you troubleshoot:");
      ui.add(eframe::egui::widgets::Spinner::new());
    });
  }
}
