use plotters::prelude::*;
use std::collections::HashMap;
use time::Date;

use super::internal::{Entries, Entry};

macro_rules! draw_triangle_series {
    ($ctx:expr, $points:expr, $name:expr, $color:expr) => {
        $ctx.draw_series(
            $points
                .into_iter()
                .map(|point| TriangleMarker::new(point, 5, $color)),
        )?
        .label($name)
        .legend(|(x, y)| TriangleMarker::new((x, y), 5, $color));
    };
}

macro_rules! draw_circle_series {
    ($ctx:expr, $points:expr, $name:expr, $color:expr) => {
        $ctx.draw_series(
            $points
                .into_iter()
                .map(|point| Circle::new(point, 5, $color)),
        )?
        .label($name)
        .legend(|(x, y)| Circle::new((x, y), 5, $color));
    };
}

pub trait LatteGraph {
    const C0: RGBColor = RGBColor(242, 213, 207);
    const C1: RGBColor = RGBColor(238, 190, 190);
    const C2: RGBColor = RGBColor(244, 184, 228);
    const C3: RGBColor = RGBColor(202, 158, 230);
    const C4: RGBColor = RGBColor(231, 130, 132);
    const C5: RGBColor = RGBColor(234, 153, 156);
    const C6: RGBColor = RGBColor(239, 159, 118);
    const C7: RGBColor = RGBColor(229, 200, 144);
    const C8: RGBColor = RGBColor(166, 209, 137);
    const C9: RGBColor = RGBColor(129, 200, 190);
    const C10: RGBColor = RGBColor(153, 209, 219);
    const C11: RGBColor = RGBColor(133, 193, 220);
    const C12: RGBColor = RGBColor(140, 170, 238);
    const C13: RGBColor = RGBColor(186, 187, 241);
    const EXCLUSIVE_MAX_COLOR_IDX: usize = 14;
    const FOREGROUND_COLOR_RGB: RGBColor = RGBColor(198, 208, 245);
    const FOREGROUND_COLOR_RGBA: RGBAColor = RGBAColor(198, 208, 245, 1.0);
    const BACKGROUND_COLOR: RGBColor = RGBColor(48, 52, 70);
    fn generate_png(&self, dates: Vec<Date>) -> anyhow::Result<()>;
}

impl LatteGraph for Entries {
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

        let mut superpoints: Vec<(String, Vec<(usize, f64)>)> = self
            .iter()
            .map(|entry| {
                (
                    entry
                        .names
                        .first()
                        .unwrap_or(&String::from("UNKNOWN"))
                        .to_owned(),
                    entry.get_points(&dates_to_usize),
                )
            })
            .collect();

        let max_y = superpoints
            .iter()
            .map(|(_, points)| {
                points
                    .iter()
                    .map(|(_, val)| *val)
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(1.0)
            })
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(1.0);
        let max_y = max_y.ceil();

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
            .draw()?;

        let mut color_idx: usize = 0;

        while let Some((name, points)) = superpoints.pop() {
            if color_idx == Self::EXCLUSIVE_MAX_COLOR_IDX {
                break;
            }
            if color_idx == 0 {
                draw_triangle_series!(ctx, points, name, Self::C0);
            } else if color_idx == 1 {
                draw_triangle_series!(ctx, points, name, Self::C1);
            } else if color_idx == 2 {
                draw_triangle_series!(ctx, points, name, Self::C2);
            } else if color_idx == 3 {
                draw_triangle_series!(ctx, points, name, Self::C3);
            } else if color_idx == 4 {
                draw_triangle_series!(ctx, points, name, Self::C4);
            } else if color_idx == 5 {
                draw_triangle_series!(ctx, points, name, Self::C5);
            } else if color_idx == 6 {
                draw_triangle_series!(ctx, points, name, Self::C6);
            } else if color_idx == 7 {
                draw_triangle_series!(ctx, points, name, Self::C7);
            } else if color_idx == 8 {
                draw_triangle_series!(ctx, points, name, Self::C8);
            } else if color_idx == 9 {
                draw_triangle_series!(ctx, points, name, Self::C9);
            } else if color_idx == 10 {
                draw_triangle_series!(ctx, points, name, Self::C10);
            } else if color_idx == 11 {
                draw_triangle_series!(ctx, points, name, Self::C11);
            } else if color_idx == 12 {
                draw_triangle_series!(ctx, points, name, Self::C12);
            } else if color_idx == 13 {
                draw_triangle_series!(ctx, points, name, Self::C13);
            }
            color_idx += 1;
        }
        color_idx = 0;

        while let Some((name, points)) = superpoints.pop() {
            if color_idx == Self::EXCLUSIVE_MAX_COLOR_IDX {
                break;
            }
            if color_idx == 0 {
                draw_circle_series!(ctx, points, name, Self::C0);
            } else if color_idx == 1 {
                draw_circle_series!(ctx, points, name, Self::C1);
            } else if color_idx == 2 {
                draw_circle_series!(ctx, points, name, Self::C2);
            } else if color_idx == 3 {
                draw_circle_series!(ctx, points, name, Self::C3);
            } else if color_idx == 4 {
                draw_circle_series!(ctx, points, name, Self::C4);
            } else if color_idx == 5 {
                draw_circle_series!(ctx, points, name, Self::C5);
            } else if color_idx == 6 {
                draw_circle_series!(ctx, points, name, Self::C6);
            } else if color_idx == 7 {
                draw_circle_series!(ctx, points, name, Self::C7);
            } else if color_idx == 8 {
                draw_circle_series!(ctx, points, name, Self::C8);
            } else if color_idx == 9 {
                draw_circle_series!(ctx, points, name, Self::C9);
            } else if color_idx == 10 {
                draw_circle_series!(ctx, points, name, Self::C10);
            } else if color_idx == 11 {
                draw_circle_series!(ctx, points, name, Self::C11);
            } else if color_idx == 12 {
                draw_circle_series!(ctx, points, name, Self::C12);
            } else if color_idx == 13 {
                draw_circle_series!(ctx, points, name, Self::C13);
            }
            color_idx += 1;
        }

        ctx.configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .border_style(Self::FOREGROUND_COLOR_RGB)
            .label_font(&Self::FOREGROUND_COLOR_RGB)
            .draw()?;

        Ok(root.present()?)
    }
}

impl LatteGraph for Entry {
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
            .map(|(_, val)| *val)
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(1.0);
        let max_y = max_y.ceil();

        let image_width: u32 = 400 + dates.len() as u32 * 100;
        let image_height: u32 = 1080;

        let filename = format!(
            "graph-{}.png",
            self.names
                .first()
                .unwrap_or(&"unkown".to_string())
                .to_lowercase()
        );

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
            .draw()?;

        ctx.draw_series(
            points
                .into_iter()
                .map(|point| TriangleMarker::new(point, 5, Self::FOREGROUND_COLOR_RGB)),
        )?;
        Ok(root.present()?)
    }
}
