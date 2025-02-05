pub mod linear_regression;
pub mod utils;
pub mod normalization;

#[cfg(test)]
mod tests {
  use super::linear_regression::LinearRegression;
  use super::utils::{load_dataset, load_params, save_params};

  #[test]
  fn test_training_and_prediction() -> Result<(), Box<dyn std::error::Error>> {
    let mut model = LinearRegression::new(Some(0.01))?;
    model.train(1000);
    let (theta0, theta1) = model.get_params();
    assert!((theta0 - 1.0).abs() < 0.1);
    assert!((theta1 - 1.0).abs() < 0.1);
    Ok(())
  }

  #[test]
  fn test_load_dataset() {
    let dataset = load_dataset().expect("Failed to load dataset");
    assert_eq!(dataset.len(), 5);
    assert_eq!(dataset[0], (10000.0, 20000.0));
  }

  #[test]
  fn test_save_and_load_params() {
    save_params(1.5, 2.5).unwrap();
    let (theta0, theta1) = load_params().unwrap();
    assert_eq!(theta0, 1.5);
    assert_eq!(theta1, 2.5);

    let path = std::env::var("THETA_PATH").unwrap();
    std::fs::remove_file(path).unwrap();
  }
}