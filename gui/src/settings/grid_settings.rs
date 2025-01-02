use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GridSettings {
  pub background_color: [f32; 3],
  pub show_grid: bool,
  pub grid_spacing: f32,
  pub grid_line_color: [f32; 3],
  pub grid_line_weight: f32,
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
      background_color: [0.2, 0.2, 0.2],
      show_grid: true,
      grid_spacing: 10.0,
      grid_line_color: [0.5, 0.5, 0.5],
      grid_line_weight: 1.0,
      lock_x_axis: true,
      lock_y_axis: true,
      ctrl_to_zoom: false,
      shift_to_horizontal: false,
      zoom_speed: 1.0,
      scroll_speed: 1.0,
    }
  }
}
