use eframe::egui::{Event, Vec2};
use crate::app::App;
use egui_plot::{Legend, Points, PlotPoints, PlotBounds};
use crate::utils::{to_rgb_color};

pub struct Plot;

impl Plot {
  // Render Plot
  pub fn render(ui: &mut eframe::egui::Ui, app: &App) {
    let grid_settings = &app.grid_settings;

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
    ui.label("Linear Regression - Gradient Descent");
    egui_plot::Plot::new("plot")
      .allow_zoom(false)
      .allow_drag(false)
      .allow_scroll(false)
      .legend(Legend::default())
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
    // let dataset = app.get_dataset();
    // let dataset_size = dataset.len();

    // egui::CentralPanel::default().show(ctx, |ui| {
      // egui::widgets::global_theme_preference_buttons(ui);

    //   let root = EguiBackend::new(ui).into_drawing_area();

    //   // Retrieve settings from the App
    //   let grid_settings = &app.grid_settings;
    //   let plot_settings = &app.plot_settings;

    //   let (x_formatter, y_formatter) = if plot_settings.swap_axes {
    //     (format_km as fn(f64) -> String, format_price as fn(f64) -> String)
    //   } else {
    //     (format_price as fn(f64) -> String, format_km as fn(f64) -> String)
    //   };

    //   // Determine whether to swap axes
    //   let swap_axes = plot_settings.swap_axes;

    //   // Dynamically determine the data bounds
    //   let (x_min, x_max, y_min, y_max) = if swap_axes {
    //     dataset
    //       .iter()
    //       .chain(app.get_predictions().iter())
    //       .fold(
    //         (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
    //         |(x_min, x_max, y_min, y_max), &(x, y)| {
    //           (x_min.min(y), x_max.max(y), y_min.min(x), y_max.max(x))
    //         },
    //       )
    //   } else {
    //     dataset
    //       .iter()
    //       .chain(app.get_predictions().iter())
    //       .fold(
    //         (f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY),
    //         |(x_min, x_max, y_min, y_max), &(x, y)| {
    //           (x_min.min(x), x_max.max(x), y_min.min(y), y_max.max(y))
    //         },
    //       )
    //   };

    //   // Fill the background with the configured color
    //   root.fill(&to_rgb_color(grid_settings.background_color)).unwrap();

    //   // Build the chart with appropriate axis labels and ranges
    //   let mut chart = ChartBuilder::on(&root)
    //     .caption("Regression Analysis", ("sans-serif", 20).into_font()
    //       .color(&to_rgb_color(grid_settings.grid_line_color)))
    //     .margin(10)
    //     .x_label_area_size(30)
    //     .y_label_area_size(30)
    //     .build_cartesian_2d(x_min..x_max, y_min..y_max)
    //     .unwrap();

    //   // Configure the grid based on the settings
    //   if grid_settings.show_grid {
    //     let grid_line_color = to_rgb_color(grid_settings.grid_line_color);

    //     chart
    //       .configure_mesh()
    //       .x_desc(if swap_axes { "Price" } else { "KM" })
    //       .y_desc(if swap_axes { "KM" } else { "Price" })
    //       .x_labels(dataset_size)
    //       .y_labels(dataset_size)
    //       .label_style(("sans-serif", 10).into_font()
    //         .color(&to_rgb_color(grid_settings.grid_line_color)))
    //       .light_line_style(ShapeStyle {
    //         color: grid_line_color.into(),
    //         filled: true,
    //         stroke_width: grid_settings.grid_line_weight as u32,
    //       })
    //       .x_label_formatter(&|value| x_formatter(*value))
    //       .y_label_formatter(&|value| y_formatter(*value))
    //       .bold_line_style(ShapeStyle {
    //         color: grid_line_color.into(),
    //         filled: true,
    //         stroke_width: grid_settings.grid_line_weight as u32 + 1,
    //       })
    //       .draw()
    //       .unwrap();
    //   }

    //   // Plot dataset points
    //   chart
    //     .draw_series(
    //       dataset
    //         .iter()
    //         .map(|&(x, y)| {
    //           let (x, y) = if swap_axes { (y, x) } else { (x, y) };
    //           Circle::new((x, y), 5, to_rgb_color(plot_settings.dataset_color).filled())
    //         }),
    //     )
    //     .unwrap()
    //     .label("Dataset")
    //     .legend(|(x, y)| Circle::new((x, y), 5, to_rgb_color(plot_settings.dataset_color).filled()));

    //   // Plot predictions
    //   chart
    //     .draw_series(
    //       app.get_predictions()
    //         .iter()
    //         .map(|&(x, y)| {
    //           let (x, y) = if swap_axes { (y, x) } else { (x, y) };
    //           Circle::new((x, y), 5, to_rgb_color(plot_settings.prediction_color).filled())
    //         }),
    //     )
    //     .unwrap()
    //     .label("Predictions")
    //     .legend(|(x, y)| Circle::new((x, y), 5, to_rgb_color(plot_settings.prediction_color).filled()));

    //   // Plot the regression line if available
    //   if let Some((theta0, theta1)) = app.get_regression_line() {
    //     chart
    //       .draw_series(LineSeries::new(
    //         (x_min as i64..=x_max as i64).map(|x| {
    //           let x = x as f64;
    //           let y = theta0 + theta1 * x;
    //           if swap_axes {
    //             (y, x)
    //           } else {
    //             (x, y)
    //           }
    //         }),
    //         to_rgb_color(plot_settings.regression_line_color),
    //       ))
    //       .unwrap()
    //       .label("Regression Line")
    //       .legend(|(x, y)| PathElement::new(
    //         vec![(x, y), (x + 20, y)],
    //         to_rgb_color(plot_settings.regression_line_color),
    //       ));
    //   }

    //   // Configure and render the legend
    //   chart
    //     .configure_series_labels()
    //     .background_style(&to_rgb_color(grid_settings.background_color).mix(0.8))
    //     .border_style(&BLACK)
    //     .draw()
    //     .unwrap();

    //   // Present the chart
    //   root.present().unwrap();
    // });