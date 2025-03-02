use linear_regression::linear_regression::LinearRegression;
use crate::settings::{GridSettings, PlotSettings, SidebarSettings, SidebarTab};
use crate::components::{Navbar, Sidebar, Plot, ModelErrorScreen};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state

#[derive(Default)]
pub struct App {
  show_sidebar: bool,
  pub sidebar_settings: SidebarSettings,
  pub grid_settings: GridSettings,
  pub plot_settings: PlotSettings,
  #[serde(skip)]
  pub error_message: Option<String>,
  #[serde(skip)]
  pub regression_model: Option<LinearRegression>, // LinearRegression instance
   #[serde(skip)]
  pub predictions: Vec<(f64, f64)>,               // Predictions for dataset
  #[serde(skip)]
  pub regression_line: Option<(f64, f64)>,
  #[serde(skip)]
  pub swapped_regression_line: Option<(f64, f64)>,
  #[serde(skip)]
  pub mae: Option<f64>, // Mean Absolute Error
  #[serde(skip)]
  pub mse: Option<f64>, // Mean Squared Error
  #[serde(skip)]
  pub rmse: Option<f64>, // Root Mean Squared Error
  #[serde(skip)]
  pub r2: Option<f64>, // R² Score
}

impl App {
  /// App constructor.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    let context = &cc.egui_ctx;

    context.tessellation_options_mut(|tess_options| {
      tess_options.feathering = false;
    });

    let mut app = Self {
      show_sidebar: true,
      sidebar_settings: SidebarSettings::default(),
      grid_settings: GridSettings::new(),
      plot_settings: PlotSettings::new(),
      error_message: None,
      regression_model: None,
      predictions: Vec::new(),
      regression_line: None,
      swapped_regression_line: None,
      mae: None,
      mse: None,
      rmse: None,
      r2: None,
    };

     // Load previous app state (if any) and override the default values.
    if let Some(storage) = cc.storage {
      if let Some(saved_app) = eframe::get_value::<Self>(storage, eframe::APP_KEY) {
        // Update the fields that can be loaded from storage.
        app.show_sidebar = saved_app.show_sidebar;
        app.sidebar_settings = saved_app.sidebar_settings;
        app.grid_settings = saved_app.grid_settings;
        app.plot_settings = saved_app.plot_settings;
      }
    }

    // Initialize the regression model.
    let (model, error_message) = match LinearRegression::new(None) {
      Ok(model) => (Some(model), None),
      Err(err) => {
        log::error!("Failed to initialize regression model: {}", err);
        (None, Some(format!("Failed to initialize model: {}", err)))
      },
    };

    app.regression_model = model;
    app.error_message = error_message;

    app
  }

  /// Reloads the LinearRegression model and updates the state.
  pub fn reload_model(&mut self) {
    match LinearRegression::new(Some(0.01)) {
      Ok(model) => {
        self.regression_model = Some(model);
        self.regression_line = None;
        self.predictions = Vec::new();
        self.error_message = None;
        self.mae = None;
        self.mse = None;
        self.rmse = None;
        self.r2 = None;
      }
      Err(err) => {
        log::error!("Failed to initialize regression model: {}", err);
        self.regression_model = None;
        self.regression_line = None;
        self.predictions = Vec::new();
        self.mae = None;
        self.mse = None;
        self.rmse = None;
        self.r2 = None;
        self.error_message = Some(format!("Failed to initialize model: {}", err));
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


  /// Computes the least squares regression line (slope and intercept)
  fn compute_regression_line(data: &Vec<(f64, f64)>) -> Option<(f64, f64)> {
    let n = data.len() as f64;
    if n < 2.0 {
      return None; // Not enough data points to compute a regression line
    }

    let sum_x: f64 = data.iter().map(|(x, _)| x).sum();
    let sum_y: f64 = data.iter().map(|(_, y)| y).sum();
    let sum_xy: f64 = data.iter().map(|(x, y)| x * y).sum();
    let sum_x2: f64 = data.iter().map(|(x, _)| x * x).sum();

    let mean_x = sum_x / n;
    let mean_y = sum_y / n;

    let numerator = sum_xy - (sum_x * mean_y);
    let denominator = sum_x2 - (sum_x * mean_x);

    let slope = if denominator.abs() > f64::EPSILON {
      numerator / denominator
    } else {
      0.0 // Avoid division by zero (e.g., vertical line case)
    };

    let intercept = mean_y - (slope * mean_x);
    Some((slope, intercept))
  }

  /// Updates predictions and regression line from the model.
  fn update_from_model(&mut self) {
    if let Some(model) = &self.regression_model {
      let dataset = model.get_dataset();

      // Ensure there are enough points to compute a regression line
      if dataset.len() < 2 {
        self.regression_line = None;
        self.swapped_regression_line = None;
        self.mae = None;
        self.mse = None;
        self.rmse = None;
        self.r2 = None;
        return;
      }

      // Generate predictions for the dataset
      self.predictions = dataset.iter().map(|&(x, _)| (x, model.predict(x))).collect();

      // Compute Normal Regression Line (y = mx + b)
      self.regression_line = Self::compute_regression_line(&self.predictions);

      let swapped_predictions = dataset.iter().map(|&(_, y)| (y, model.predict(y))).collect();
      self.swapped_regression_line = Self::compute_regression_line(&swapped_predictions);
      let (mae, mse, rmse, r2) = model.compute_precision();
      self.mae = Some(mae);
      self.mse = Some(mse);
      self.rmse = Some(rmse);
      self.r2 = Some(r2);
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

  // Function to change tab
  pub fn set_sidebar_current_tab(&mut self, tab: SidebarTab) {
    self.sidebar_settings.current_tab = tab
  }

  /// Returns a reference to the dataset.
  pub fn get_dataset(&self) -> Vec<(f64, f64)> {
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
}

impl eframe::App for App {
  /// Called by the frame work to save state before shutdown.
  fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, eframe::APP_KEY, self);
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    self.plot_settings.need_auto_bounds = false;
    egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
      ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
          Navbar::render(ui, self);
        });
        ui.add_space(ui.available_width());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
          egui::widgets::global_theme_preference_buttons(ui);
        });
      });
    });

    egui::SidePanel::right("sidebar")
      .show_animated(ctx, !self.show_sidebar, |ui| {
        Sidebar::render(ui, self);
    });

    
    egui::CentralPanel::default().show(ctx, |ui| {
      if let Some(_) = self.regression_model {
        Plot::render(ui, self);
      } else {
        ModelErrorScreen::render(ui, self);
      }
    });
  }
}
