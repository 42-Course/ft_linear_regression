use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizationFactors {
  pub x_min: f64,
  pub x_max: f64,
  pub y_min: f64,
  pub y_max: f64,
}

impl Default for NormalizationFactors {
  fn default() -> Self {
    Self {
      x_min: 0.0,
      x_max: 1.0,
      y_min: 0.0,
      y_max: 1.0,
    }
  }
}

impl NormalizationFactors {
  /// Computes normalization factors from the dataset
  pub fn from_data(data: &[(f64, f64)]) -> Self {
    let x_min = data.iter().map(|&(x, _)| x).fold(f64::INFINITY, f64::min);
    let x_max = data.iter().map(|&(x, _)| x).fold(f64::NEG_INFINITY, f64::max);
    let y_min = data.iter().map(|&(_, y)| y).fold(f64::INFINITY, f64::min);
    let y_max = data.iter().map(|&(_, y)| y).fold(f64::NEG_INFINITY, f64::max);

    Self { x_min, x_max, y_min, y_max }
  }

  /// Denormalizes a mileage value
  pub fn denormalize_x(&self, x: f64) -> f64 {
    x * (self.x_max - self.x_min) + self.x_min
  }

  /// Denormalizes a predicted price
  pub fn denormalize_y(&self, y: f64) -> f64 {
    y * (self.y_max - self.y_min) + self.y_min
  }
}
