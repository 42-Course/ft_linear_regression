use std::error::Error;
use std::path::Path;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use dotenv::dotenv;
use csv::ReaderBuilder;
use crate::normalization::NormalizationFactors;

/// Retrieves the dataset path from the `.env` file.
pub fn get_dataset_path() -> Result<String, Box<dyn Error>> {
  dotenv().ok();

  env::var("DATASET_PATH").or_else(|_| Ok("./data/data.csv".to_string()))
}

/// Loads and parses the dataset using the `csv` library, skipping the header row.
pub fn load_dataset_file() -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
  let path = get_dataset_path()?;

  let mut reader = ReaderBuilder::new().has_headers(true).from_path(Path::new(&path))?;

  let mut dataset = Vec::new();
  for result in reader.records() {
    let record = result?;
    if let (Some(km), Some(price)) = (record.get(0), record.get(1)) {
      dataset.push((km.parse()?, price.parse()?));
    }
  }

  Ok(dataset)
}

/// Loads the dataset and falls back to default values
pub fn load_dataset() -> Vec<(f64, f64)> {
  load_dataset_file().unwrap_or_else(|_| {
        vec![
            (240000.0, 3650.0),
            (139800.0, 3800.0),
            (150500.0, 4400.0),
            (185530.0, 4450.0),
            (176000.0, 5250.0),
            (114800.0, 5350.0),
            (166800.0, 5800.0),
            (89000.0, 5990.0),
            (144500.0, 5999.0),
            (84000.0, 6200.0),
            (82029.0, 6390.0),
            (63060.0, 6390.0),
            (74000.0, 6600.0),
            (97500.0, 6800.0),
            (67000.0, 6800.0),
            (76025.0, 6900.0),
            (48235.0, 6900.0),
            (93000.0, 6990.0),
            (60949.0, 7490.0),
            (65674.0, 7555.0),
            (54000.0, 7990.0),
            (68500.0, 7990.0),
            (22899.0, 7990.0),
            (61789.0, 8290.0),
        ]
    })
}

/// Normalizes both `km` and `price`.
pub fn normalize_dataset(data: &[(f64, f64)], factors: &NormalizationFactors) -> Vec<(f64, f64)> {
  data.iter()
    .map(|&(x, y)| (
      (x - factors.x_min) / (factors.x_max - factors.x_min),
      (y - factors.y_min) / (factors.y_max - factors.y_min)
    ))
    .collect()
}

/// Retrieves the theta file path from the `.env` file.
/// Creates it if it doesn't exist.
pub fn get_theta_path() -> io::Result<String> {
  dotenv().ok();
  let path = env::var("THETA_PATH").unwrap_or("./data/theta.txt".to_string());

  if !Path::new(&path).exists() {
    File::create(&path)?;
  }

  Ok(path)
}

/// Saves the theta parameters to the file
pub fn save_params(theta0: f64, theta1: f64) -> io::Result<()> {
  let path = get_theta_path()?;
  let mut file = File::create(path)?;
  writeln!(file, "{},{}", theta0, theta1)?;
  Ok(())
}

/// Loads the theta files if they exist
pub fn load_params() -> io::Result<(f64, f64)> {
  let path = get_theta_path()?;
  let file = File::open(path)?;
  let line = io::BufReader::new(file).lines().next().unwrap_or(Ok("0.0,0.0".to_string()))?;
  let params: Vec<f64> = line.split(',').filter_map(|v| v.trim().parse::<f64>().ok()).collect();
  if params.len() != 2 {
    return Err(io::Error::new(io::ErrorKind::InvalidData, "Theta file corrupt or missing values"));
  }
  Ok((params[0], params[1]))
}
