use crate::app::egui::Visuals;
use eframe::egui;
use crate::settings::{GridSettings, PlotSettings, SidebarSettings, SidebarTab};
use crate::components::{Navbar, Plane, Sidebar};
use linear_regression::linear_regression::LinearRegression;

pub struct App {
  show_sidebar: bool,
  pub regression_model: Option<LinearRegression>, // LinearRegression instance
  pub predictions: Vec<(f64, f64)>,               // Predictions for dataset
  pub regression_line: Option<(f64, f64)>,        // θ₀ and θ₁
  pub sidebar_settings: SidebarSettings,
  pub grid_settings: GridSettings,
  pub plot_settings: PlotSettings,
  pub plane: Plane,
}

impl App {
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    let context = &cc.egui_ctx;

    context.tessellation_options_mut(|tess_options| {
      tess_options.feathering = false;
    });
    
    context.set_visuals(Visuals::dark());

    let model = LinearRegression::new(None).ok();
    Self {
      show_sidebar: true,
      regression_model: model,
      predictions: Vec::new(),
      regression_line: None,
      sidebar_settings: SidebarSettings::default(),
      grid_settings: GridSettings::new(),
      plot_settings: PlotSettings::new(),
      plane: Plane::new(), 
    }
  }
}

impl App {
  /// Reloads the LinearRegression model and updates the state.
  pub fn reload_model(&mut self) {
    match LinearRegression::new(Some(0.01)) {
      Ok(model) => {
        self.regression_model = Some(model);
        self.regression_line = None;
        self.predictions = Vec::new();
      }
      Err(e) => {
        eprintln!("Failed to reload model: {}", e);
      }
    }
  }

  /// Trains the model and updates the GUI state.
  pub fn train_model(&mut self) {
    if let Some(model) = &mut self.regression_model {
      model.train(1000); // Train for 1000 iterations
      self.update_from_model();
    }
  }

  /// Toggle sidebar
  pub fn toggle_sidebar(&mut self) {
      self.show_sidebar = !self.show_sidebar   
  }

  /// Get the grid settingss
  pub fn get_grid_settings(&mut self) -> &mut GridSettings {
    &mut self.grid_settings
  }

  /// Get the plot settings
  pub fn get_plot_settings(&mut self) -> &mut PlotSettings {
    &mut self.plot_settings
  }

  /// Get the sidebar settings
  pub fn get_sidebar_current_tab(&mut self) -> SidebarTab {
    self.sidebar_settings.current_tab
  }

  pub fn set_sidebar_current_tab(&mut self, tab: SidebarTab) {
    self.sidebar_settings.current_tab = tab
  }

  /// Returns a reference to the dataset.
  pub fn get_dataset(&self) -> &Vec<(f64, f64)> {
    self.regression_model.as_ref().expect("Regression model is not initialized").get_dataset()
  }

  /// Returns a reference to the predictions.
  pub fn get_predictions(&self) -> &Vec<(f64, f64)> {
    &self.predictions
  }

  /// Returns a reference to the regression line coefficients (theta0, theta1).
  pub fn get_regression_line(&self) -> Option<(f64, f64)> {
    self.regression_line
  }

  /// Updates predictions and regression line from the model.
  fn update_from_model(&mut self) {
    if let Some(model) = &self.regression_model {
      self.predictions = model
        .get_dataset()
        .iter()
        .map(|&(x, _)| (x, model.predict(x)))
        .collect();
      self.regression_line = Some(model.get_params());
    }
  }
}

impl eframe::App for App {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
      Navbar::render(ui, self);
    });

    egui::SidePanel::right("sidebar")
      .show_animated(ctx, self.show_sidebar, |ui| {
        Sidebar::render(ui, self);
    });

    // println!("\n[{:?}]\n", self.regression_model.as_ref().unwrap().get_dataset());

    self.plane.render(ctx, self);
  }
}
