use std::env;
use std::thread::sleep;
use std::time::Duration;
use linear_regression::linear_regression::LinearRegression;
use linear_regression::utils::save_params;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let iterations: usize = env::args()
        .nth(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(42);

    let mut model = LinearRegression::new(Some(1.0))?;

    let pb = ProgressBar::new(iterations as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} Calculating price... [{bar:40.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("##-"),
    );

    for i in 0..iterations {
        model.train(1);
        pb.set_position(i as u64);
    }
    sleep(Duration::from_millis(42));
    pb.set_position(iterations as u64);

    let (theta0, theta1) = model.get_params();
    save_params(theta0, theta1)?;

    println!(
        "Training complete! Parameters saved:\nθ₀ = {:.4}\nθ₁ = {:.4}",
        theta0, theta1
    );
    Ok(())
}
