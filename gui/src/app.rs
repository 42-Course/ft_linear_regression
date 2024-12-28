use crate::components::{plot::Plot, controls::Controls, status::Status};
use eframe::egui;

pub struct App {
    pub theta0: f64,
    pub theta1: f64,
    pub dataset: Vec<(f64, f64)>,
    pub training_progress: Option<f32>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            theta0: 0.0,
            theta1: 0.0,
            dataset: vec![],
            training_progress: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Linear Regression Trainer");
            ui.separator();
            Controls::show(ui, self);
            ui.separator();
            Plot::show(ui, &self.dataset, self.theta0, self.theta1);
            ui.separator();
            Status::show(ui, self.training_progress);
        });
    }
}
