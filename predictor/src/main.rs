use std::fs::File;
use serde_json::Value;

fn main() {
    println!("Enter mileage: ");
    let mut mileage = String::new();
    std::io::stdin().read_line(&mut mileage).unwrap();
    let mileage: f64 = mileage.trim().parse().unwrap();

    // Load model parameters
    let file = File::open("../data/model.json").expect("Model file not found");
    let model: Value = serde_json::from_reader(file).expect("Failed to parse model file");

    let theta0 = model["theta0"].as_f64().unwrap();
    let theta1 = model["theta1"].as_f64().unwrap();

    let price = linear_regression::predict(theta0, theta1, mileage);
    println!("Estimated price: {:.2}", price);
}
