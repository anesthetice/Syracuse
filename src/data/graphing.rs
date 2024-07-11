use anyhow::Context;
use crossterm::style::Stylize;
use itertools::Itertools;
use plotters::prelude::*;
use std::path::PathBuf;

use super::{internal::{Entries, Entry}, syrtime::SyrDate};
use crate::{config::Config, info, warn};

trait GraphMethods {
    fn get_points(&self, dates: &[SyrDate]) -> Vec<(f64, f64)>;
}

impl GraphMethods for Entry {
    fn get_points(&self, dates: &[SyrDate]) -> Vec<(f64, f64)> {
        dates.iter().enumerate().map(|(idx, date)| {
            match self.blocs.get(date) {
                Some(nanos) => {
                    // idx + 1 since we pad our graph and 0 is not used
                    ((idx+1) as f64, *nanos as f64 / 36e11)
                },
                None => ((idx+1) as f64, 0_f64),
            }
        }).collect_vec()
    }
}

pub fn graph(entries: Entries, start_date: SyrDate, end_date: SyrDate) -> anyhow::Result<()> {
    info!("initializing...");
    let bg_rgb = rgb_translate(Config::get().graph_background_rgb);
    let fg_rgb = rgb_translate(Config::get().graph_foreground_rgb);
    let coarse_grid_rgb = rgb_translate(Config::get().graph_coarse_grid_rgb);
    let fine_grid_rgb = rgb_translate(Config::get().graph_fine_grid_rgb);
    let marker_size = Config::get().graph_marker_size;

    let marker_color_wheel: Vec<RGBColor> = Config::get().graph_marker_rgb.clone().into_iter().map(rgb_translate).collect();
    let mcw_len = marker_color_wheel.len();
    let mut mcw_idx: usize = 0;

    if marker_color_wheel.is_empty() {
        Err(crate::error::Error{}).context("please provide at least one color in graph_marker_colors")?;
    }

    let dates = SyrDate::expand_from_bounds(start_date, end_date);

    if dates.len() < 3 {
        Err(crate::error::Error{}).context("at minimum, a span of three days is required to build a graph")?
    }

    let mut superpoints: Vec<(String, Vec<(f64, f64)>)> = entries.iter().map(|entry| (entry.name.clone(), entry.get_points(&dates))).collect();

    let mut sum_points: Vec<(f64, f64)> = superpoints[0].1.clone();
    for (_, points) in superpoints.iter().skip(1) {
        for (idx, point) in points.iter().enumerate() {
            sum_points[idx].1 += point.1
        }
    }
    let max_y = sum_points.iter().map(|&(_, a)| a).max_by(|a, b| a.total_cmp(b)).unwrap_or(6.0).ceil();

    let image_width: u32 = dates.len() as u32 * 100 + 500;
    let image_height: u32 = 1080;

    let filepath = {
        let path = Config::get().graph_output_dir.clone();
        if path.is_empty() {
            PathBuf::from("graph.png")
        } else {
            let path = PathBuf::from(path);
            if path.is_dir() {
                path.join("graph.png")
            } else {
                warn!("invalid directory, defaulting to current directory");
                PathBuf::from("graph.png")
            }
        }
    };

    info!("drawing...");
    let root =
        BitMapBackend::new(&filepath, (image_width, image_height)).into_drawing_area();
    root.fill::<RGBColor>(&bg_rgb)?;

    let mut ctx = ChartBuilder::on(&root)
        .margin_top(30)
        .margin_right(30)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        // ignore 0 and pad by 2 to the right
        .build_cartesian_2d(0_f64..(dates.len()+2) as f64, 0_f64..max_y)?;

    ctx.configure_mesh()
        .axis_style(ShapeStyle {
            color: fg_rgb.to_rgba(),
            filled: true,
            stroke_width: 2,
        })
        .label_style(("sans-serif", 20).with_color(fg_rgb.to_rgba()))
        .x_label_formatter(&|v| {
            let v_idx = *v as usize;
            if v_idx <= dates.len() && v_idx > 0 {
                dates[v_idx - 1].to_string()
            } else {
                String::with_capacity(0)
            }
        })
        // due to padding (1 on the left, 2 on the right)
        .x_labels(dates.len()+3)
        .bold_line_style(coarse_grid_rgb.to_rgba().stroke_width(2))
        .light_line_style(fine_grid_rgb.to_rgba().stroke_width(1))
        .draw()?;

    match Config::get().graph_interpolation_method {
        interpolation::InterpolationMethod::Linear => {
            ctx.draw_series(
                interpolation::linear(sum_points)
                    .into_iter()
                    .map(|coord| Circle::new(coord, 0, fg_rgb.stroke_width(2))),
            )?;
        }, 
        interpolation::InterpolationMethod::Makima => {
            ctx.draw_series(
                interpolation::makima(sum_points)
                    .into_iter()
                    .map(|coord| Circle::new(coord, 0, fg_rgb.stroke_width(2))),
            )?;
        }
    }

    for (_, points) in superpoints.iter_mut() {
        points.retain(|(_, y)| *y != 0.0)
    }
    superpoints.retain(|(_, points)| !points.is_empty());

    while let Some((name, points)) = superpoints.pop() {
        let color = marker_color_wheel[mcw_idx];
        ctx.draw_series(
            points
                .into_iter()
                .map(|coord| 
                    Circle::new(
                        coord,
                        marker_size,
                        color.stroke_width(2)
                    )
                ),
        )?
        .label(name)
        .legend(move |point| Circle::new(point, marker_size, color.stroke_width(2)));

        mcw_idx += 1;
        if mcw_idx == mcw_len {
            mcw_idx = 0;
            break;
        }
    }

    while let Some((name, points)) = superpoints.pop() {
        let color = marker_color_wheel[mcw_idx];
        ctx.draw_series(
            points
                .into_iter()
                .map(|coord|
                    TriangleMarker::new(
                        coord,
                        marker_size,
                        color.stroke_width(2)
                    )
                ),
        )?
        .label(name)
        .legend(move |coord| TriangleMarker::new(coord, marker_size, color.stroke_width(2)));

        mcw_idx += 1;
        if mcw_idx == mcw_len {
            mcw_idx = 0;
            break;
        }
    }

    while let Some((name, points)) = superpoints.pop() {
        let color = marker_color_wheel[mcw_idx];
        ctx.draw_series(
            points
                .into_iter()
                .map(|coord|
                    Cross::new(
                        coord,
                        marker_size,
                        color.stroke_width(2)
                    )
                ),
        )?
        .label(name)
        .legend(move |coord| Cross::new(coord, marker_size, color.stroke_width(2)));

        mcw_idx += 1;
        if mcw_idx == mcw_len {
            break;
        }
    }

    if !superpoints.is_empty() {
        warn!("failed to graph every single entry, consider adding more colors in the config or tightening the date span");
    }

    ctx.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(fg_rgb)
        .margin(15)
        .label_font(("sans-serif", 15.0).with_color(fg_rgb))
        .draw()?;

    info!("saving...");
    Ok(root.present()?)
}

fn rgb_translate(rgb: (u8, u8, u8)) -> RGBColor {
    RGBColor(rgb.0, rgb.1, rgb.2)
}

pub mod interpolation {
    use crossterm::style::Stylize;
    use itertools::Itertools;
    use serde::{Deserialize, Serialize};

    use crate::{config::Config, warn};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub(crate) enum InterpolationMethod {
        Linear,
        Makima,
    }

    // plotters.rs has a lineseries options but I dislike it as the width is not consistent depending on the slope
    // this method gives us a more consistent line width
    pub(super) fn linear(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        let nb_points = Config::get().graph_nb_interpolated_points;
        points
            .into_iter()
            .tuple_windows()
            .map(|((x_i, y_i), (x_ip1, y_ip1))| {
                let mut local_points: Vec<(f64, f64)> = Vec::new();

                // lots of "off-by-one" bs but it's not really important
                let step_size = (x_ip1 - x_i) / nb_points as f64;
                let mut x = x_i;
                while x < x_ip1 {
                    local_points.push((x, (y_i*(x_ip1 - x) + y_ip1*(x - x_i))/(x_ip1 - x_i)));
                    x += step_size;
                }
                local_points
            }).fold(Vec::new(), |mut global_points, local_points| {
                global_points.extend(local_points); 
                global_points
            })
    }


    pub(super) fn makima(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        if points.len() < 5 {
            warn!("failed to interpolate data using makima, defaulting to linear, {} out of the 5 required points met", points.len());
            return linear(points);
        }   
        let n = points.len() - 1;

        // slope between each point
        let m: Vec<f64> = points
            .iter()
            .tuple_windows()
            .map(|(&(x_i, y_i), &(x_ip1, y_ip1))| {
                (y_ip1 - y_i) / (x_ip1 - x_i)
            })
            .collect();

        // spline slopes
        let mut s: Vec<f64> = Vec::new();
        // deals with the two first spline slopes
        s.push(m[0]); s.push((m[0] + m[1])/2.0);
        for i in 2..points.len()-2 {
            s.push({
                let w_1 = (m[i+1] - m[i]).abs() + (m[i+1] + m[i]).abs()/2.0;
                let w_2 = (m[i-1] - m[i-2]).abs() + (m[i-1] + m[i-2]).abs()/2.0;
                (w_1 / (w_1 + w_2)) * m[i-1] + (w_2 / (w_1 + w_2)) * m[i]
            });
        }
        // deals with the last two spline slopes
        s.push((m[n-3] + m[n-2])/2.0); s.push(m[n-1]);

        let nb_points = Config::get().graph_nb_interpolated_points;

        points
            .into_iter()
            .tuple_windows::<(_, _)>()
            .enumerate()
            .map(|(i, ((x_i, y_i), (x_ip1, _)))| {
                let a_i = y_i;
                let b_i = s[i];
                let c_i = (3.0*m[i] - 2.0*s[i] - s[i+1]) / (x_ip1 - x_i);
                let d_i = (s[i] + s[i+1] - 2.0*m[i]) / (x_ip1 - x_i)*(x_ip1 - x_i);

                let mut local_points: Vec<(f64, f64)> = Vec::new();

                // lots of "off-by-one" bs but it's not really important
                let step_size = (x_ip1 - x_i) / nb_points as f64;
                let mut x = x_i;
                while x < x_ip1 {
                    let y = a_i + b_i * (x-x_i) + c_i * (x-x_i)*(x-x_i) + d_i * (x-x_i)*(x-x_i)*(x-x_i);
                    local_points.push(
                        (x, if y < 0.0 {0.0} else {y})
                    );
                    x += step_size;
                }
                local_points
            })
            .fold(Vec::new(), |mut global_points, local_points| {
                global_points.extend(local_points); 
                global_points
            })
    }
}