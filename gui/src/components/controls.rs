use eframe::egui;

pub struct Controls;

impl Controls {
    pub fn show(ui: &mut egui::Ui, app: &mut crate::app::App) {
        ui.add(egui::Slider::new(&mut app.theta0, 0.0..=1.0).text("Theta0"));
    }
}
