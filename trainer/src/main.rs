use std::thread::sleep;
use std::time::Duration;
use linear_regression::linear_regression::LinearRegression;
use linear_regression::utils::save_params;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let mut model = LinearRegression::new(Some(0.001))?;

  let iterations = 42;

  let pb = ProgressBar::new(iterations);
  pb.set_style(
    ProgressStyle::default_bar()
    .template("{spinner:.green} Calculating price... [{bar:40.cyan/blue}] {pos}%")
      .unwrap()
      .progress_chars("##-"),
  );

  for i in 0..iterations {
    model.train(1);
    pb.set_position(i);
  }
  sleep(Duration::from_millis(42));
  pb.set_position(iterations);

  let (theta0, theta1) = model.get_params();
  save_params(theta0, theta1)?;

  println!(
    "Training complete! Parameters saved:\nθ₀ = {:.4}\nθ₁ = {:.4}",
    theta0, theta1
  );
  Ok(())
}
