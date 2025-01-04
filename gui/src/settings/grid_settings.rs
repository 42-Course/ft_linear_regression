use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GridSettings {
  pub show_grid: bool,
  pub labels_color: [f32; 3],
  pub lock_x_axis: bool,
  pub lock_y_axis: bool,
  pub ctrl_to_zoom: bool,
  pub shift_to_horizontal: bool,
  pub zoom_speed: f32,
  pub scroll_speed: f32,
}

impl GridSettings {
  pub fn new() -> Self {
    Self {
      show_grid: true,
      labels_color: [0.5, 0.5, 0.5],
      lock_x_axis: true,
      lock_y_axis: true,
      ctrl_to_zoom: false,
      shift_to_horizontal: false,
      zoom_speed: 1.0,
      scroll_speed: 1.0,
    }
  }
}
