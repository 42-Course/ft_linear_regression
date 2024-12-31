use std::error::Error;
use std::path::Path;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use dotenv::dotenv;
use csv::ReaderBuilder;

/// Retrieves the dataset path from the `.env` file.
pub fn get_dataset_path() -> Result<String, Box<dyn Error>> {
  dotenv().ok(); // Load environment variables
  env::var("DATASET_PATH").map_err(|_| "DATASET_PATH not set in .env".into())
}

/// Loads and parses the dataset using the `csv` library, skipping the header row.
pub fn load_dataset() -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
  let path = get_dataset_path()?; // Retrieve the dataset path

  // Open the CSV file and parse it, skipping the headers
  let mut reader = ReaderBuilder::new()
    .has_headers(true)
    .from_path(Path::new(&path))?;

  let mut dataset = Vec::new();
  for result in reader.records() {
    let record = result?;
    if let (Some(km), Some(price)) = (record.get(0), record.get(1)) {
      dataset.push((km.parse()?, price.parse()?));
    }
  }

  Ok(dataset)
}

pub fn normalize_dataset(data: &[(f64, f64)]) -> Vec<(f64, f64)> {
  let x_min = data.iter().map(|&(x, _)| x).fold(f64::INFINITY, f64::min);
  let x_max = data.iter().map(|&(x, _)| x).fold(f64::NEG_INFINITY, f64::max);

  data.iter()
    .map(|&(x, y)| ((x - x_min) / (x_max - x_min), y))
    .collect()
}

/// Retrieves the theta file path from the `.env` file.
/// Creates it if it doesn't exist
pub fn get_theta_path() -> io::Result<String> {
  dotenv().ok();
  let path = env::var("THETA_PATH").map_err(|_| io::Error::new(io::ErrorKind::NotFound, "THETA_PATH not set"))?;

  if !std::path::Path::new(&path).exists() {
    std::fs::File::create(&path)?;
  }

  Ok(path)
}

pub fn save_params(theta0: f64, theta1: f64) -> io::Result<()> {
  let path = get_theta_path()?;
  let mut file = File::create(path)?;
  writeln!(file, "{},{}", theta0, theta1)?;
  Ok(())
}

pub fn load_params() -> io::Result<(f64, f64)> {
  let path = get_theta_path()?;
  let file = File::open(path)?;
  let line = io::BufReader::new(file).lines().next().unwrap_or(Ok("0.0,0.0".to_string()))?;
  let params: Vec<f64> = line
    .split(',')
    .filter_map(|v| v.trim().parse::<f64>().ok())
    .collect();
  Ok((params[0], params[1]))
}

// let file = File::open(path)?;

// It is equivalent to:

// let file = match File::open(path) {
//     Ok(f) => f,
//     Err(e) => return Err(e),
// };
