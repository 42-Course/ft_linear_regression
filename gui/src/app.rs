use eframe::egui::{self, CentralPanel};
use egui_plotter::EguiBackend;
use linear_regression::linear_regression::LinearRegression;
use crate::settings::{GridSettings, PlotSettings, SidebarSettings, SidebarTab};
use crate::components::{Navbar, Sidebar, Plot};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state


#[derive(Default)]
pub struct App {
  show_sidebar: bool,
  pub regression_model: Option<LinearRegression>, // LinearRegression instance
  pub predictions: Vec<(f64, f64)>,               // Predictions for dataset
  pub regression_line: Option<(f64, f64)>,
  pub sidebar_settings: SidebarSettings,
  pub grid_settings: GridSettings,
  pub plot_settings: PlotSettings,
}

impl App {
  /// App constructor.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    let context = &cc.egui_ctx;

    context.tessellation_options_mut(|tess_options| {
      tess_options.feathering = false;
    });

    // Load previous app state (if any).
    if let Some(storage) = cc.storage {
      return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
    }

    let model = LinearRegression::new(None).ok();
    Self {
      show_sidebar: true,
      regression_model: model,
      predictions: Vec::new(),
      regression_line: None,
      sidebar_settings: SidebarSettings::default(),
      grid_settings: GridSettings::new(),
      plot_settings: PlotSettings::new(),
    }
  }

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
}

impl eframe::App for App {
  /// Called by the frame work to save state before shutdown.
  fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, eframe::APP_KEY, self);
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("navbar").show(ctx, |ui| {
      Navbar::render(ui, self);
    });

    egui::SidePanel::right("sidebar")
      .show_animated(ctx, !self.show_sidebar, |ui| {
        Sidebar::render(ui, self);
    });

    // Plot::render(ctx, self);


    // CentralPanel::default().show(ctx, |ui| {
      // let root = EguiBackend::new(ui).into_drawing_area();
      // root.fill(&WHITE).unwrap();
      // let mut chart = ChartBuilder::on(&root)
      //   .caption("y=x^2", ("sans-serif", 50).into_font())
      //   .margin(5)
      //   .x_label_area_size(30)
      //   .y_label_area_size(30)
      //   .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
      //   .unwrap();

      // chart.configure_mesh().draw().unwrap();

    // });
  }
}

// egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
//   egui::menu::bar(ui, |ui| {
//     let is_web = cfg!(target_arch = "wasm32");
//     if !is_web {
//       ui.menu_button("File", |ui| {
//         if ui.button("Quit").clicked() {
//           ctx.send_viewport_cmd(egui::ViewportCommand::Close);
//         }
//       });
//       ui.add_space(16.0);
//     }

//     egui::widgets::global_theme_preference_buttons(ui);
//   });
// });