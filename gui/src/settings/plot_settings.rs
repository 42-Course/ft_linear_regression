use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct PlotSettings {
  pub dataset_color: [f32; 3],
  pub prediction_color: [f32; 3],
  pub error_line_color: [f32; 3],
  pub error_line_weight: f32,
  pub regression_line_color: [f32; 3],
  pub regression_line_weight: f32,
  pub swap_axes: bool,
  pub need_auto_bounds: bool,
}

impl PlotSettings {
  pub fn new() -> Self {
    Self {
      dataset_color: [1.0, 0.0, 0.0],
      prediction_color: [0.0, 0.0, 1.0],
      error_line_color: [1.0, 0.0, 1.0],
      error_line_weight: 1.0,
      regression_line_color: [0.0, 1.0, 0.0],
      regression_line_weight: 1.0,
      swap_axes: false,
      need_auto_bounds: true,
    }
  }

  pub fn update_dataset_color(&mut self, color: [f32; 3]) {
    self.dataset_color = color;
  }

  pub fn update_prediction_color(&mut self, color: [f32; 3]) {
    self.prediction_color = color;
  }

  pub fn toggle_swap_axes(&mut self) {
    self.swap_axes = !self.swap_axes;
    self.need_auto_bounds = true;
  }

  pub fn update_error_line_color(&mut self, color: [f32; 3]) {
    self.error_line_color = color;
  }

  pub fn set_error_line_weight(&mut self, weight: f32) {
    self.error_line_weight = weight;
  }

  pub fn update_regression_line_color(&mut self, color: [f32; 3]) {
    self.regression_line_color = color;
  }

  pub fn set_regression_line_weight(&mut self, weight: f32) {
    self.regression_line_weight = weight;
  }
}
