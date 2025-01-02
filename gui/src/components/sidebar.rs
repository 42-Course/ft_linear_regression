use egui::DragValue;
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
    let grid_settings = app.get_grid_settings();

    ui.heading("Grid Settings");
    ui.checkbox(&mut grid_settings.show_grid, "Show Grid").on_hover_text("Check to display the plot grid.");
    ui.checkbox(&mut grid_settings.lock_x_axis, "Lock X-Axis").on_hover_text("When checked the X axis is fixed, zoom wouldn't affect.");
    ui.checkbox(&mut grid_settings.lock_y_axis, "Lock Y-Axis").on_hover_text("When checked the Y axis is fixed, zoom wouldn't affect.");
    ui.separator();
    ui.horizontal(|ui| {
      ui.label("Background Color:").on_hover_text("Grid Background Color.");
      let mut color = grid_settings.background_color;
      if ui.color_edit_button_rgb(&mut color).changed() {
        grid_settings.background_color = color;
      }
    });
    ui.horizontal(|ui| {
      ui.label("Grid Line Color:").on_hover_text("Color of the grid lines.");
      let mut color = grid_settings.grid_line_color;
      if ui.color_edit_button_rgb(&mut color).changed() {
        grid_settings.grid_line_color = color;
      }
    });
    ui.horizontal(|ui| {
      ui.label("Grid Spacing:").on_hover_text("The space between the grid lines in pixels.");
      let mut spacing = grid_settings.grid_spacing;
      if ui.add(egui::Slider::new(&mut spacing, 5.0..=50.0)).changed() {
        grid_settings.grid_spacing = spacing;
      }
    });
    ui.horizontal(|ui| {
      ui.label("Grid Line Weight:").on_hover_text("The thickness of the grid lines.");
      let mut line_weight = grid_settings.grid_line_weight;
      if ui.add(egui::Slider::new(&mut line_weight, 0.1..=5.0)).changed() {
        grid_settings.grid_line_weight = line_weight;
      } 
    });
    ui.separator();
    ui.checkbox(&mut grid_settings.ctrl_to_zoom, "Ctrl to zoom").on_hover_text("If unchecked, the behavior of the Ctrl key is inverted compared to the default controls\ni.e., scrolling the mouse without pressing any keys zooms the plot");
    ui.checkbox(&mut grid_settings.shift_to_horizontal, "Shift for horizontal scroll").on_hover_text("If unchecked, the behavior of the shift key is inverted compared to the default controls\ni.e., hold to scroll vertically, release to scroll horizontally");
    ui.horizontal(|ui| {
        ui.add(
            DragValue::new(&mut grid_settings.zoom_speed)
                .range(0.1..=2.0)
                .speed(0.1),
        );
        ui.label("Zoom speed").on_hover_text("How fast to zoom in and out with the mouse wheel");
    });
    ui.horizontal(|ui| {
        ui.add(
            DragValue::new(&mut grid_settings.scroll_speed)
                .range(0.1..=100.0)
                .speed(0.1),
        );
        ui.label("Scroll speed").on_hover_text("How fast to pan with the mouse wheel");
    });
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
