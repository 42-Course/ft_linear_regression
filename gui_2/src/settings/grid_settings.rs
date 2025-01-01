pub struct GridSettings {
  pub background_color: [f32; 3],
  pub show_grid: bool,
  pub grid_spacing: f32,
  pub grid_line_color: [f32; 3],
  pub grid_line_weight: f32,
  pub show_x_axis: bool,
  pub show_y_axis: bool,
}

impl GridSettings {
  pub fn new() -> Self {
    Self {
      background_color: [0.2, 0.2, 0.2],
      show_grid: true,
      grid_spacing: 10.0,
      grid_line_color: [0.5, 0.5, 0.5],
      grid_line_weight: 1.0,
      show_x_axis: true,
      show_y_axis: true,
    }
  }

  pub fn update_background_color(&mut self, color: [f32; 3]) {
    self.background_color = color;
  }

  pub fn toggle_grid(&mut self) {
    self.show_grid = !self.show_grid;
  }

  pub fn set_grid_spacing(&mut self, spacing: f32) {
    self.grid_spacing = spacing;
  }

  pub fn update_grid_line_color(&mut self, color: [f32; 3]) {
    self.grid_line_color = color;
  }

  pub fn set_grid_line_weight(&mut self, weight: f32) {
    self.grid_line_weight = weight;
  }

  pub fn toggle_x_axis(&mut self) {
    self.show_x_axis = !self.show_x_axis;
  }

  pub fn toggle_y_axis(&mut self) {
    self.show_y_axis = !self.show_y_axis;
  }
}
