pub fn predict(theta0: f64, theta1: f64, mileage: f64) -> f64 {
    theta0 + theta1 * mileage
}

pub fn gradient_descent(
    _x: &[f64],
    _y: &[f64],
    _alpha: f64,
    _iterations: usize
) -> (f64, f64) {
    // Implement gradient descent here
    return (12.2, 42.2)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
