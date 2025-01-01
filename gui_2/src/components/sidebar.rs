use eframe::egui;
use crate::app::App;
use crate::settings::SidebarTab;

pub struct Sidebar;

impl Sidebar {
  /// Renders the sidebar tab and content.
  pub fn render(ui: &mut eframe::egui::Ui, app: &mut App) {
    ui.vertical(|ui| {
      Sidebar::render_tabs(ui, app);
      ui.separator();

      match app.get_sidebar_current_tab() {
        SidebarTab::GridSettings => Sidebar::render_grid_settings(ui, app),
        SidebarTab::PlotSettings => Sidebar::render_plot_settings(ui, app),
      }
    });
  }

  /// Renders the tab selection at the top of the sidebar.
  fn render_tabs(ui: &mut eframe::egui::Ui, app: &mut App) {
    ui.horizontal(|ui| {
      if ui.selectable_label(app.get_sidebar_current_tab() == SidebarTab::GridSettings, "Grid Settings").clicked() {
        app.set_sidebar_current_tab(SidebarTab::GridSettings);
      }

      if ui.selectable_label(app.get_sidebar_current_tab() == SidebarTab::PlotSettings, "Plot Settings").clicked() {
        app.set_sidebar_current_tab(SidebarTab::PlotSettings);
      }
    });
  }

  /// Renders the controls for grid settings.
  fn render_grid_settings(ui: &mut eframe::egui::Ui, app: &mut App) {
    ui.heading("Grid Settings");
    let grid_settings = app.get_grid_settings();

    ui.horizontal(|ui| {
      ui.label("Background Color:");
      let mut color = grid_settings.background_color;
      if ui.color_edit_button_rgb(&mut color).changed() {
        grid_settings.background_color = color;
      }
    });

    let mut show_grid = grid_settings.show_grid;
    if ui.checkbox(&mut show_grid, "Show Grid").changed() {
      grid_settings.toggle_grid();
    }

    let mut spacing = grid_settings.grid_spacing;
    if ui.add(egui::Slider::new(&mut spacing, 5.0..=50.0).text("Grid Spacing")).changed() {
      grid_settings.grid_spacing = spacing;
    }

    ui.horizontal(|ui| {
      ui.label("Grid Line Color:");
      let mut color = grid_settings.grid_line_color;
      if ui.color_edit_button_rgb(&mut color).changed() {
        grid_settings.grid_line_color = color;
      }
    });

    let mut line_weight = grid_settings.grid_line_weight;
    if ui.add(egui::Slider::new(&mut line_weight, 0.5..=5.0).text("Grid Line Weight")).changed() {
      grid_settings.grid_line_weight = line_weight;
    }

    let mut show_x_axis = grid_settings.show_x_axis;
    if ui.checkbox(&mut show_x_axis, "Show X-Axis").changed() {
      grid_settings.toggle_x_axis();
    }

    let mut show_y_axis = grid_settings.show_y_axis;
    if ui.checkbox(&mut show_y_axis, "Show Y-Axis").changed() {
      grid_settings.toggle_y_axis();
    }
  }

  /// Renders the controls for plot settings and updates the App's PlotSettings.
  fn render_plot_settings(ui: &mut eframe::egui::Ui, app: &mut App) {
    ui.heading("Plot Settings");
    let plot_settings = app.get_plot_settings();

    ui.horizontal(|ui| {
      ui.label("Dataset Color:");
      let mut color = plot_settings.dataset_color;
      if ui.color_edit_button_rgb(&mut color).changed() {
        plot_settings.dataset_color = color;
      }
    });

    ui.horizontal(|ui| {
      ui.label("Prediction Color:");
      let mut color = plot_settings.prediction_color;
      if ui.color_edit_button_rgb(&mut color).changed() {
        plot_settings.prediction_color = color;
      }
    });

    let mut show_error_lines = plot_settings.show_error_lines;
    if ui.checkbox(&mut show_error_lines, "Show Error Lines").changed() {
      plot_settings.show_error_lines = !plot_settings.show_error_lines;
    }

    ui.horizontal(|ui| {
      ui.label("Error Line Color:");
      let mut color = plot_settings.error_line_color;
      if ui.color_edit_button_rgb(&mut color).changed() {
        plot_settings.error_line_color = color;
      }
    });

    let mut error_line_weight = plot_settings.error_line_weight;
    if ui.add(egui::Slider::new(&mut error_line_weight, 0.5..=5.0).text("Error Line Weight")).changed() {
      plot_settings.error_line_weight = error_line_weight;
    }

    ui.horizontal(|ui| {
      ui.label("Regression Line Color:");
      let mut color = plot_settings.regression_line_color;
      if ui.color_edit_button_rgb(&mut color).changed() {
        plot_settings.regression_line_color = color;
      }
    });

    let mut regression_line_weight = plot_settings.regression_line_weight;
    if ui.add(egui::Slider::new(&mut regression_line_weight, 0.5..=5.0).text("Regression Line Weight")).changed() {
      plot_settings.regression_line_weight = regression_line_weight;
    }

    let mut swap_axes = plot_settings.swap_axes;
    if ui.checkbox(&mut swap_axes, "Swap Axes").changed() {
      plot_settings.toggle_swap_axes();
    }

  }
}
