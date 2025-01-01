use num_format::{Locale, ToFormattedString};
use plotters::prelude::RGBColor;

/// Converts an `[f32; 3]` array representing a color into `RGBColor`.
/// The input values should be in the range `[0.0, 1.0]`.
pub fn to_rgb_color(color: [f32; 3]) -> RGBColor {
  RGBColor(
    (color[0] * 255.0) as u8,
    (color[1] * 255.0) as u8,
    (color[2] * 255.0) as u8,
  )
}

// Format a large number with sufix
pub fn format_large_number(value: i64) -> String { 
  let abs_value = value.abs() as u64;
  let formatted = match abs_value {
    1_000_000.. => format!("{:.1}m", abs_value as f64 / 1_000_000.0),
    1_000.. => format!("{:.1}k", abs_value as f64 / 1_000.0),
    _ => abs_value.to_formatted_string(&Locale::en),
  };
  format!("{}{}", if value < 0 { "-" } else { "" }, formatted)
}


/// Formatter for price (e.g., "$3.6m" or "$15,000")
pub fn format_price(value: f64) -> String {
  format!("${}", format_large_number(value as i64))
}

/// Formatter for kilometers (e.g., "3.6m km" or "15,000 km")
pub fn format_km(value: f64) -> String {
  format!("{} km", format_large_number(value as i64))
}