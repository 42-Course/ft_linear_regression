use eframe::egui::Ui;

pub struct Plot;

impl Plot {
    pub fn show(ui: &mut Ui, _dataset: &[(f64, f64)], _theta0: f64, _theta1: f64) {
        ui.label("Dataset and Regression Line");
        // Use Plotters to draw the chart
    }
}
