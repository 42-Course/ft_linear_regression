// use std::fs::File;
// use std::io::Write;
use csv::ReaderBuilder;

fn main() {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("../data/dataset.csv")
        .expect("Failed to load dataset");

    let mut x = Vec::new();
    let mut y = Vec::new();

    for result in rdr.records() {
        let record = result.expect("Error reading record");
        x.push(record[0].parse::<f64>().unwrap());
        y.push(record[1].parse::<f64>().unwrap());
    }

    // let (theta0, theta1) = linear_regression::gradient_descent(&x, &y, 0.01, 1000);

    // let mut file = File::create("../data/model.json").expect("Unable to create model file");
    // write!(
    //     file,
    //     r#"{{"theta0": {}, "theta1": {}}}"#,
    //     theta0, theta1
    // )
    // .expect("Failed to write model file");
}
