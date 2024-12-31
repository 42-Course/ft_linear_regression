use crate::utils::{load_dataset};

#[derive(Debug, Default)]
pub struct LinearRegression {
  theta0: f64,
  theta1: f64,
  learning_rate: f64,
  data: Vec<(f64, f64)>,
  costs: Vec<f64>,
}

impl LinearRegression {
  /// Creates a new LinearRegression model with an optional learning rate.
  pub fn new(learning_rate: Option<f64>) -> Result<Self, Box<dyn std::error::Error>> {
    let data = load_dataset()?;

    Ok(Self {
      theta0: 0.0,
      theta1: 0.0,
      learning_rate: learning_rate.unwrap_or(0.001),
      data,
      costs: Vec::new(),
    })
  }

  /// Trains the model using gradient descent.
  pub fn train(&mut self, iterations: usize) {
    let m = self.data.len() as f64;

    for i in 0..iterations {
      // Compute the current cost before updating
      let current_cost = self.compute_cost();

      // Pre-iteration checks
      if current_cost.is_nan() || current_cost.is_infinite() {
        eprintln!(
            "Iteration {}: Cost is NaN or infinite. Stopping training.",
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

      // Update θ₀ and θ₁
      self.theta0 -= self.learning_rate * (1.0 / m) * sum_error_theta0;
      self.theta1 -= self.learning_rate * (1.0 / m) * sum_error_theta1;

      // Compute and save the cost
      self.costs.push(self.compute_cost());
    }
    println!("({:?}, {:?})", &self.theta0, &self.theta1);
  }

  /// Computes the cost function J(θ).
  pub fn compute_cost(&self) -> f64 {
    let m = self.data.len() as f64;
    self.data
      .iter()
      .map(|&(x, y)| {
        let prediction = self.theta0 + self.theta1 * x;
        (prediction - y).powi(2)
      })
      .sum::<f64>()
      / (2.0 * m)
  }

  /// Predicts the price for a given mileage.
  pub fn predict(&self, mileage: f64) -> f64 {
    self.theta0 + self.theta1 * mileage
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
  pub fn get_dataset(&self) -> &Vec<(f64, f64)> {
    &self.data
  }
}
