pub fn normalize(data: &[f64]) -> Vec<f64> {
    let max = data.iter().cloned().fold(f64::MIN, f64::max);
    let min = data.iter().cloned().fold(f64::MAX, f64::min);
    data.iter().map(|x| (x - min) / (max - min)).collect()
}
