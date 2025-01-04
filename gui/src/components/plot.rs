use eframe::egui::{Event, Vec2};
use crate::app::App;
use egui_plot::{Legend, Points, PlotPoints, PlotBounds};
use crate::utils::{to_rgb_color, format_price, format_km};

pub struct Plot;

impl Plot {
  // Render Plot
  pub fn render(ui: &mut eframe::egui::Ui, app: &App) {
    let grid_settings = &app.grid_settings;
    let plot_settings = &app.plot_settings;

    let (scroll, pointer_down, modifiers) = ui.input(|i| {
      let scroll = i.events.iter().find_map(|e| match e {
        Event::MouseWheel {
          unit: _,
          delta,
          modifiers: _,
        } => Some(*delta),
        _ => None,
      });
      (scroll, i.pointer.primary_down(), i.modifiers)
    });
    let (x_label, y_label, x_formatter, y_formatter) = if plot_settings.swap_axes {
      ("Price", "KM", format_price as fn(f64) -> String, format_km as fn(f64) -> String)
    } else {
      ("KM", "Price", format_km as fn(f64) -> String, format_price as fn(f64) -> String)
    };
    ui.label("Linear Regression - Gradient Descent");
    egui_plot::Plot::new("Linear Regression Plot")
      .allow_zoom(false)
      .allow_drag(false)
      .allow_scroll(false)
      .show_grid(grid_settings.show_grid)
      .set_margin_fraction(Vec2 { x: 0.1, y: 0.1 })
      .legend(Legend::default())
      .x_axis_label(eframe::egui::RichText::new(x_label)
        .strong()
        .color(to_rgb_color(grid_settings.labels_color)))
      .y_axis_label(eframe::egui::RichText::new(y_label)
        .strong()
        .color(to_rgb_color(grid_settings.labels_color)))
      .x_axis_formatter(|x, _| {
        x_formatter(x.value)
      })
      .y_axis_formatter(|y, _| {
        y_formatter(y.value)
      })
      .show(ui, |plot_ui| {
      if let Some(mut scroll) = scroll {
        if modifiers.ctrl == grid_settings.ctrl_to_zoom {
          scroll = Vec2::splat(scroll.x + scroll.y);
          let mut zoom_factor = Vec2::from([
            (scroll.x * grid_settings.zoom_speed / 10.0).exp(),
            (scroll.y * grid_settings.zoom_speed / 10.0).exp(),
          ]);
          if grid_settings.lock_x_axis {
            zoom_factor.x = 1.0;
          }
          if grid_settings.lock_y_axis {
            zoom_factor.y = 1.0;
          }
          plot_ui.zoom_bounds_around_hovered(zoom_factor);
        } else {
          if modifiers.shift == grid_settings.shift_to_horizontal {
            scroll = Vec2::new(scroll.y, scroll.x);
          }
          if grid_settings.lock_x_axis {
            scroll.x = 0.0;
          }
          if grid_settings.lock_y_axis {
            scroll.y = 0.0;
          }
          let delta_pos = grid_settings.scroll_speed * scroll;
          plot_ui.translate_bounds(delta_pos);
        }
      }
      if plot_ui.response().hovered() && pointer_down {
        let mut pointer_translate = -plot_ui.pointer_coordinate_drag_delta();
        if grid_settings.lock_x_axis {
          pointer_translate.x = 0.0;
        }
        if grid_settings.lock_y_axis {
          pointer_translate.y = 0.0;
        }
        plot_ui.translate_bounds(pointer_translate);
      }
      Plot::plot_dataset(plot_ui, app);
    });
  }

  pub fn plot_dataset(plot_ui: &mut egui_plot::PlotUi, app: &App) {
    // Get the dataset and settings
    let dataset = app.get_dataset();
    let predictions = app.get_predictions();
    let plot_settings = &app.plot_settings;
    
    if dataset.is_empty() {
      return;
    }

    // Dynamically determine the data bounds
    let (x_min, x_max, y_min, y_max) = if plot_settings.swap_axes {
      dataset
        .iter()
        .chain(app.get_predictions().iter())
        .fold(
          (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
          |(x_min, x_max, y_min, y_max), &(x, y)| {
            (x_min.min(y), x_max.max(y), y_min.min(x), y_max.max(x))
          },
        )
    } else {
      dataset
        .iter()
        .chain(app.get_predictions().iter())
        .fold(
          (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
          |(x_min, x_max, y_min, y_max), &(x, y)| {
            (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
          },
        )
    };

    // Prepare dataset points based on axis swap
    let dataset_points: Vec<[f64; 2]> = if plot_settings.swap_axes {
      dataset.iter().map(|&(x, y)| [y, x]).collect()
    } else {
      dataset.iter().map(|&(x, y)| [x, y]).collect()
    };

    let predictions_points: Vec<[f64; 2]> = if plot_settings.swap_axes {
      predictions.iter().map(|&(x, y)| [y, x]).collect()
    } else {
      predictions.iter().map(|&(x, y)| [x, y]).collect()
    };

    // Render the dataset and predictions as points
    plot_ui.points(
      Points::new(PlotPoints::from(dataset_points))
        .color(to_rgb_color(plot_settings.dataset_color))
        .radius(3.0)
        .name("Dataset"),
    );

    plot_ui.points(
      Points::new(PlotPoints::from(predictions_points))
        .color(to_rgb_color(plot_settings.prediction_color))
        .radius(3.0)
        .name("Prediction"),
    );

    if plot_settings.need_auto_bounds {
      plot_ui.set_plot_bounds(PlotBounds::from_min_max([x_min, y_min], [x_max, y_max]));
    }
  }
}