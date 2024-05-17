use anyhow::Context;
use itertools::Itertools;
use plotters::prelude::*;
use super::{internal::{Entries, Entry}, syrtime::SyrDate};
use crate::config::Config;

trait GraphMethods {
    fn get_points(&self, dates: &[SyrDate]) -> Vec<(f64, f64)>;
}

impl GraphMethods for Entry {
    fn get_points(&self, dates: &[SyrDate]) -> Vec<(f64, f64)> {
        dates.into_iter().enumerate().map(|(idx, date)| {
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
    let bg_rgb = rgb_translate(Config::get().graph_background_rgb);
    let fg_rgb = rgb_translate(Config::get().graph_foreground_rgb);
    let coarse_grid_rgb = rgb_translate(Config::get().graph_coarse_grid_rgb);
    let fine_grid_rgb = rgb_translate(Config::get().graph_fine_grid_rgb);
    let marker_colors: Vec<RGBColor> = Config::get().graph_marker_colors.clone().into_iter().map(|rgb| rgb_translate(rgb)).collect();

    let dates = SyrDate::expand_from_bounds(start_date, end_date);

    if dates.len() < 3 {
        Err(crate::error::Error{}).context("failed to generate graph, three day span minimum required")?
    }

    let superpoints: Vec<(String, Vec<(f64, f64)>)> = entries.iter().map(|entry| (entry.name.clone(), entry.get_points(&dates))).collect();

    let mut sum_points: Vec<(f64, f64)> = superpoints[0].1.clone();
    for (_, points) in superpoints.iter().skip(1) {
        for (idx, point) in points.into_iter().enumerate() {
            sum_points[idx].1 += point.1
        }
    }
    println!("{:?}", sum_points);
    let mut meta_sum_points: Vec<Vec<(f64, f64)>> = Vec::new();
    {
        println!();
        sum_points.split(|(x, y)| *y==0.0).for_each(|point| {print!("{point:?} ")});
        println!();

        let mut buffer: Vec<(f64, f64)> = Vec::new();
        let mut stop_signal: bool = sum_points[1].1 == 0.0;
        for (x, y) in sum_points.iter() {
            if *y == 0.0 {
                if stop_signal && !buffer.is_empty() {
                    meta_sum_points.push(buffer.clone());
                    buffer.clear();
                } else if !stop_signal {
                    buffer.push((*x, *y))
                }
                stop_signal = true;
            } else {
                stop_signal = false;
                buffer.push((*x, *y))
            }
        }
        meta_sum_points.push(buffer);
        /*
        let filtered_data: Vec<(f64, f64)> = sum_points
            .iter()
            .copied()
            .enumerate()
            .filter(|&(i, (_, y))| {
                if y == 0.0 {
                    i == 0 || i == sum_points.len()-1 || sum_points[i - 1].1 != 0.0 || sum_points[i + 1].1 != 0.0
                }
                else {
                    true
                }
            })
            .map(|(_, point)| point)
            .collect();
        println!("{:?}", filtered_data);
        */
    }
    println!("{:?}", meta_sum_points);

    let image_width: u32 = dates.len() as u32 * 100 + 500;
    let image_height: u32 = 1080;

    let root =
        BitMapBackend::new("graph.png", (image_width, image_height)).into_drawing_area();
    root.fill::<RGBColor>(&bg_rgb)?;

    let mut ctx = ChartBuilder::on(&root)
        .margin_top(30)
        .margin_right(30)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        // ignore 0 and pad by 2 to the right
        .build_cartesian_2d(0_f64..(dates.len()+2) as f64, -6_f64..6_f64)?;

    ctx.configure_mesh()
        .axis_style(ShapeStyle {
            color: fg_rgb.to_rgba(),
            filled: true,
            stroke_width: 2,
        })
        .label_style(("sans-serif", 20).with_color(fg_rgb.to_rgba()))
        .x_label_formatter(&|v| {
            if v.fract() == 0.0 && *v as usize <= dates.len() && *v as usize > 0 {
                dates[*v as usize - 1].to_string()
            } else {
                String::with_capacity(0)
            }
        })
        .x_labels(dates.len())
        .bold_line_style(
            ShapeStyle {
                color: coarse_grid_rgb.to_rgba(),
                filled: false,
                stroke_width: 2
            })
        .light_line_style(ShapeStyle {
            color: fine_grid_rgb.to_rgba(),
            filled: false,
            stroke_width: 1
        })
        .draw()?;

    ctx.draw_series(LineSeries::new(sum_points.into_iter(), fg_rgb.stroke_width(2)))?;

    ctx.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .border_style(fg_rgb)
        .margin(15)
        .label_font(("sans-serif", 15.0).with_color(fg_rgb))
        .draw()?;

    Ok(root.present()?)
}

fn rgb_translate(rgb: (u8, u8, u8)) -> RGBColor {
    RGBColor(rgb.0, rgb.1, rgb.2)
}

mod interpolation {
    use anyhow::Context;
    use itertools::Itertools;

    type interpolator = Vec<Box<dyn Fn(f64) -> Option<f64>>>;

    fn makima(points: Vec<(f64, f64)>) -> anyhow::Result<()> {

        if points.len() < 5 {
            Err(crate::error::Error{}).context("makima interpolation requires at least 5 points")?   
        }

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
        for idx in 2..points.len()-2 {
            
        }


        Ok(())
    }
}