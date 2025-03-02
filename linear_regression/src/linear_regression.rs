use crate::utils::{load_dataset, normalize_dataset};
use serde::{Serialize, Deserialize};
use crate::normalization::NormalizationFactors;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LinearRegression {
  theta0: f64,
  theta1: f64,
  learning_rate: f64,
  data: Vec<(f64, f64)>,
  costs: Vec<f64>,
  normalization: NormalizationFactors,
}

impl LinearRegression {
  /// Creates a new LinearRegression model with an optional learning rate.
  pub fn new(learning_rate: Option<f64>) -> Result<Self, Box<dyn std::error::Error>> {
    let data = load_dataset();

    let factors = NormalizationFactors::from_data(&data);
    let normalized_data = normalize_dataset(&data, &factors);

    Ok(Self {
        theta0: 0.0,
        theta1: 0.0,
        learning_rate: learning_rate.unwrap_or(0.001),
        data: normalized_data,
        costs: Vec::new(),
        normalization: factors,
    })
  }


  /// Trains the model using gradient descent.
  pub fn train(&mut self, iterations: usize) {
    let m = self.data.len() as f64;
    if m == 0.0 {
      eprintln!("Error: No dataset loaded.");
      return;
    }

    for i in 0..iterations {
      let current_cost = self.compute_cost();

      // Stop if cost is NaN or if the reduction is negligible
      if current_cost.is_nan() || current_cost.is_infinite() || 
         (i > 0 && (self.costs[i - 1] - current_cost).abs() < 1e-6) {
        eprintln!(
            "Iteration {}: Cost converged or is NaN. Stopping training.",
            i
        );
        break;
      }

      let mut sum_error_theta0 = 0.0;
      let mut sum_error_theta1 = 0.0;

      for &(mileage, price) in &self.data {
        let error = (self.theta0 + self.theta1 * mileage) - price;
        sum_error_theta0 += error;
        sum_error_theta1 += error * mileage;
      }

      let alpha_div_m = self.learning_rate / m;
      self.theta0 -= alpha_div_m * sum_error_theta0;
      self.theta1 -= alpha_div_m * sum_error_theta1;

      // Store cost history
      self.costs.push(current_cost);
    }
  }

  /// Computes the cost function J(θ).
  pub fn compute_cost(&self) -> f64 {
    let m = self.data.len() as f64;
    if m == 0.0 {
      return f64::NAN;
    }

    self.data
      .iter()
      .map(|&(x, y)| {
        let prediction = self.theta0 + self.theta1 * x;
        (prediction - y).powi(2)
      })
      .sum::<f64>()
      / (2.0 * m)
  }

  pub fn compute_precision(&self) -> (f64, f64, f64, f64) {
    let dataset = self.get_dataset();
    let n = dataset.len() as f64;

    if dataset.is_empty() {
      return (f64::NAN, f64::NAN, f64::NAN, f64::NAN);
    }

    let mut sum_absolute_error = 0.0;
    let mut sum_squared_error = 0.0;
    let mut sum_total_variance = 0.0;
    
    let mean_y = dataset.iter().map(|&(_, y)| y).sum::<f64>() / n;

    for &(x, real_y) in &dataset {
      let predicted_y = self.predict(x);
      let error = real_y - predicted_y;
      sum_absolute_error += error.abs();
      sum_squared_error += error.powi(2);
      sum_total_variance += (real_y - mean_y).powi(2);
    }

    let mae = sum_absolute_error / n;
    let mse = sum_squared_error / n;
    let rmse = mse.sqrt();
    let r2 = 1.0 - (sum_squared_error / sum_total_variance);

    (mae, mse, rmse, r2)
  }

  /// Predicts the price for a given mileage.
  pub fn predict(&self, mileage: f64) -> f64 {
    if self.theta0 == 0f64 && self.theta1 == 0f64 {
      return 0.0f64;
    }
    let normalized_x = (mileage - self.normalization.x_min) / (self.normalization.x_max - self.normalization.x_min);
    let normalized_y = self.theta0 + self.theta1 * normalized_x;
    self.normalization.denormalize_y(normalized_y)
  }

  /// Returns the model's parameters.
  pub fn get_params(&self) -> (f64, f64) {
    (self.theta0, self.theta1)
  }

  /// Sets the model's parameters.
  pub fn set_params(&mut self, theta0: f64, theta1: f64) {
    self.theta0 = theta0;
    self.theta1 = theta1;
  }
  
  /// Returns a reference to the dataset.
  pub fn get_normalized_dataset(&self) -> &Vec<(f64, f64)> {
    &self.data
  }

  /// Returns a reference to the dataset.
  pub fn get_dataset(&self) -> Vec<(f64, f64)> {
    // self.data.clone()
    self.data
      .iter()
      .map(|&(x, y)| (
        self.normalization.denormalize_x(x),
        self.normalization.denormalize_y(y)
      ))
      .collect()
  }
}
