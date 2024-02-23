use plotters::prelude::*;
use std::collections::HashMap;
use time::Date;

use super::internal::{Entries, Entry};

pub trait Graph {
    // amaranth pink
    const C0: RGBColor = RGBColor(243, 167, 186);
    // cocktail red
    const C1: RGBColor = RGBColor(253, 109, 114);
    // deep saffron
    const C2: RGBColor = RGBColor(255, 150, 58);
    // corn
    const C3: RGBColor = RGBColor(250, 234, 93);
    // mountain lake green
    const C4: RGBColor = RGBColor(117, 185, 150);
    // ceulean
    const C5: RGBColor = RGBColor(0, 143, 190);

    const EXCLUSIVE_MAX_COLOR_IDX: usize = 6;
    const FOREGROUND_COLOR_RGB: RGBColor = RGBColor(205, 214, 244);
    const FOREGROUND_COLOR_RGBA: RGBAColor = RGBAColor(205, 214, 244, 1.0);
    const BACKGROUND_COLOR: RGBColor = RGBColor(30, 30, 46);
    const SUM_LINE_COLROR: RGBColor = RGBColor(166, 173, 200);
    const BOLD_LINE_COLOR: RGBColor = RGBColor(84, 87, 108);
    const LIGHT_LINE_COLOR: RGBColor = RGBColor(49, 50, 68);
    fn generate_png(&self, dates: Vec<Date>) -> anyhow::Result<()>;
}

impl Graph for Entries {
    fn generate_png(&self, mut dates: Vec<Date>) -> anyhow::Result<()> {
        if dates.is_empty() {
            Err(crate::error::Error::InvalidInput)?
        }
        // padding the dates
        dates.insert(
            0,
            dates.first().unwrap().previous_day().unwrap_or(Date::MIN),
        );
        dates.push(dates.last().unwrap().next_day().unwrap_or(Date::MAX));

        let dates_to_usize: HashMap<Date, usize> = {
            let mut out = HashMap::new();
            for (idx, date) in dates.clone().into_iter().enumerate().skip(1) {
                if date == *dates.last().unwrap() {
                    break;
                }
                out.insert(date, idx);
            }
            out
        };

        let usize_to_dates: HashMap<usize, String> = {
            let mut out = HashMap::new();
            for (idx, date) in dates.clone().into_iter().enumerate().skip(1) {
                if date == *dates.last().unwrap() {
                    break;
                }
                out.insert(
                    idx,
                    format!(
                        "{:0>2}/{:0>2}/{:0>4}",
                        date.day(),
                        date.month() as u8,
                        date.year()
                    ),
                );
            }
            out
        };

        let mut sum_points: Vec<(usize, f64)> =
            (0_usize..dates.len() - 1).map(|num| (num, 0.0)).collect();

        let mut superpoints: Vec<(String, Vec<(usize, f64)>)> = self
            .iter()
            .map(|entry| {
                (
                    entry
                        .names
                        .first()
                        .unwrap_or(&String::from("UNKNOWN"))
                        .to_owned(),
                    entry
                        .get_points(&dates_to_usize)
                        .into_iter()
                        .map(|(idx, val)| {
                            sum_points.get_mut(idx).unwrap().1 += val;
                            (idx, val)
                        })
                        .collect(),
                )
            })
            .collect();

        superpoints.retain(|(_, points)| !points.is_empty());
        superpoints.reverse();

        let max_y = sum_points
            .iter()
            .map(|(_, y)| *y)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(1.0)
            .ceil();

        let image_width: u32 = 400 + dates.len() as u32 * 100;
        let image_height: u32 = 1080;

        let root =
            BitMapBackend::new("graph-all.png", (image_width, image_height)).into_drawing_area();
        root.fill::<RGBColor>(&Self::BACKGROUND_COLOR)?;

        let mut ctx = ChartBuilder::on(&root)
            .margin_top(30)
            .margin_right(30)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..dates.len() - 1, 0.0_f64..max_y)?;

        ctx.configure_mesh()
            .axis_style(ShapeStyle {
                color: Self::FOREGROUND_COLOR_RGBA,
                filled: true,
                stroke_width: 2,
            })
            .label_style(("sans-serif", 20).with_color(Self::FOREGROUND_COLOR_RGBA))
            .x_label_formatter(&|v| match usize_to_dates.get(v) {
                Some(string) => string.to_owned(),
                None => String::with_capacity(0),
            })
            .x_labels(dates.len())
            .bold_line_style(Self::BOLD_LINE_COLOR)
            .light_line_style(Self::LIGHT_LINE_COLOR)
            .draw()?;

        ctx.draw_series(LineSeries::new(
            sum_points.into_iter(),
            Self::SUM_LINE_COLROR.stroke_width(2),
        ))?;

        let mut color_idx: usize = 0;

        let colors = [Self::C0, Self::C1, Self::C2, Self::C3, Self::C4, Self::C5];

        while let Some((name, points)) = superpoints.pop() {
            ctx.draw_series(
                points
                    .into_iter()
                    .map(|point| TriangleMarker::new(point, 6, colors[color_idx])),
            )?
            .label(name)
            .legend(move |point| TriangleMarker::new(point, 6, colors[color_idx]));

            color_idx += 1;
            if color_idx == Self::EXCLUSIVE_MAX_COLOR_IDX {
                break;
            }
        }
        color_idx = 0;

        while let Some((name, points)) = superpoints.pop() {
            ctx.draw_series(
                points
                    .into_iter()
                    .map(|point| Circle::new(point, 6, colors[color_idx].stroke_width(2))),
            )?
            .label(name)
            .legend(move |point| Circle::new(point, 6, colors[color_idx].stroke_width(2)));

            color_idx += 1;
            if color_idx == Self::EXCLUSIVE_MAX_COLOR_IDX {
                break;
            }
        }
        color_idx = 0;

        while let Some((name, points)) = superpoints.pop() {
            ctx.draw_series(
                points
                    .into_iter()
                    .map(|point| Cross::new(point, 6, colors[color_idx].stroke_width(2))),
            )?
            .label(name)
            .legend(move |point| Cross::new(point, 6, colors[color_idx].stroke_width(2)));

            color_idx += 1;
            if color_idx == Self::EXCLUSIVE_MAX_COLOR_IDX {
                break;
            }
        }

        ctx.configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .border_style(Self::FOREGROUND_COLOR_RGB)
            .margin(15)
            .label_font(("sans-serif", 15.0).with_color(Self::FOREGROUND_COLOR_RGB))
            .draw()?;

        Ok(root.present()?)
    }
}

impl Graph for Entry {
    fn generate_png(&self, mut dates: Vec<Date>) -> anyhow::Result<()> {
        if dates.is_empty() {
            Err(crate::error::Error::InvalidInput)?
        }
        // padding the dates
        dates.insert(
            0,
            dates.first().unwrap().previous_day().unwrap_or(Date::MIN),
        );
        dates.push(dates.last().unwrap().next_day().unwrap_or(Date::MAX));

        let dates_to_usize: HashMap<Date, usize> = {
            let mut out = HashMap::new();
            for (idx, date) in dates.clone().into_iter().enumerate().skip(1) {
                if date == *dates.last().unwrap() {
                    break;
                }
                out.insert(date, idx);
            }
            out
        };

        let usize_to_dates: HashMap<usize, String> = {
            let mut out = HashMap::new();
            for (idx, date) in dates.clone().into_iter().enumerate().skip(1) {
                if date == *dates.last().unwrap() {
                    break;
                }
                out.insert(
                    idx,
                    format!(
                        "{:0>2}/{:0>2}/{:0>4}",
                        date.day(),
                        date.month() as u8,
                        date.year()
                    ),
                );
            }
            out
        };
        let points = self.get_points(&dates_to_usize);

        let max_y = points
            .iter()
            .map(|(_, y)| *y)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(1.0)
            .ceil();

        let image_width: u32 = 400 + dates.len() as u32 * 100;
        let image_height: u32 = 1080;

        let name = self
            .names
            .first()
            .unwrap_or(&"unkown".to_string())
            .to_lowercase();

        let filename = format!("graph-{}.png", name);

        let root = BitMapBackend::new(&filename, (image_width, image_height)).into_drawing_area();
        root.fill::<RGBColor>(&Self::BACKGROUND_COLOR)?;

        let mut ctx = ChartBuilder::on(&root)
            .margin_top(30)
            .margin_right(30)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..dates.len() - 1, 0.0_f64..max_y)?;

        ctx.configure_mesh()
            .axis_style(ShapeStyle {
                color: Self::FOREGROUND_COLOR_RGBA,
                filled: true,
                stroke_width: 2,
            })
            .label_style(("sans-serif", 20).with_color(Self::FOREGROUND_COLOR_RGBA))
            .x_label_formatter(&|v| match usize_to_dates.get(v) {
                Some(string) => string.to_owned(),
                None => String::with_capacity(0),
            })
            .x_labels(dates.len())
            .bold_line_style(Self::BOLD_LINE_COLOR)
            .light_line_style(Self::LIGHT_LINE_COLOR)
            .draw()?;

        ctx.draw_series(
            points
                .into_iter()
                .map(|point| TriangleMarker::new(point, 6, Self::FOREGROUND_COLOR_RGB)),
        )?
        .label(name)
        .legend(move |point| TriangleMarker::new(point, 6, Self::FOREGROUND_COLOR_RGB));

        ctx.configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .border_style(Self::FOREGROUND_COLOR_RGB)
            .margin(15)
            .label_font(("sans-serif", 15.0).with_color(Self::FOREGROUND_COLOR_RGB))
            .draw()?;

        Ok(root.present()?)
    }
}
