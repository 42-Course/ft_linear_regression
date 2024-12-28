use eframe::egui::Ui;

pub struct Status;

impl Status {
    pub fn show(ui: &mut Ui, progress: Option<f32>) {
        if let Some(p) = progress {
            ui.label(format!("Training Progress: {:.2}%", p * 100.0));
        } else {
            ui.label("Ready");
        }
    }
}
