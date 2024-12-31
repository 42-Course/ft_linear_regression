use linear_regression::linear_regression::LinearRegression;
use linear_regression::utils::load_params;
use inquire::Text;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;


fn main() -> Result<(), Box<dyn std::error::Error>> {
  let (theta0, theta1) = load_params()?;

  let mut model = LinearRegression::new(None)?;
  model.set_params(theta0, theta1);

  let mileage_input = Text::new("Enter mileage (in kilometers):")
    .with_placeholder("e.g., 420000")
    .prompt()?;

  let mileage: f64 = mileage_input.trim().parse()?;

  let pb = ProgressBar::new(42);
  pb.set_style(
    ProgressStyle::default_bar()
      .template("{spinner:.green} Calculating price... [{bar:40.cyan/blue}] {pos}%")
      .unwrap()
      .progress_chars("##-"),
  );

  for i in 0..42 {
    pb.set_position(i);
    sleep(Duration::from_millis(10));
  }

  let estimated_price = model.predict(mileage);

  println!(
    "\n🚗 The estimated price for a car with mileage {:.2} km is: {:.2} 💸",
    mileage, estimated_price
  );

  Ok(())
}