use plotters::{prelude::*, style::full_palette::{ORANGE, PINK, PURPLE, TEAL}};
use std::collections::HashMap;
use time::Date;

use super::internal::{Entries, Entry};

const COLORS: [RGBColor; 10] = [RED, PINK, MAGENTA, PURPLE, BLUE, CYAN, TEAL, GREEN, YELLOW, ORANGE];

pub trait Export {
    fn generate_png(&self, start_date: Date, end_date: Date) -> anyhow::Result<()>;
}

impl Export for Entries {
    fn generate_png(&self, start_date: Date, end_date: Date) -> anyhow::Result<()> {
        let dates = crate::utils::expand_dates(&start_date, &end_date);
        let dates_to_usize: HashMap<Date, usize> = {
            let mut out = HashMap::new();
            for (idx, date) in dates.clone().into_iter().enumerate() {
                out.insert(date, idx+1);
            }
            out
        };

        let root_area = BitMapBackend::new("export.png", (1920, 1080))
            .into_drawing_area();
        root_area.fill(&WHITE)?;

        let mut ctx = ChartBuilder::on(&root_area)
            .margin_top(30)
            .margin_right(30)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..dates.len()+1, 0.0_f64..12.0_f64)?;

        ctx
            .configure_mesh()
            .draw()?;

        let mut points_list: Vec<(String, Vec<(usize, f64)>)> = self.iter().map(|entry| {
            (entry.names.get(0).unwrap_or(&String::default()).to_owned(), entry.get_points(&dates_to_usize))}
        ).collect();


        let mut color_idx: usize = 0; let max_color_idx = COLORS.len(); let mut switch: u8 = 0;
        while let Some((name, points)) = points_list.pop() {
            if color_idx == max_color_idx {
                color_idx = 0;
                switch += 1;
            }
            if switch == 0 {
                if color_idx == 0 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, RED)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, RED));
                }
                else if color_idx == 1 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, PINK)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, PINK));
                }
                else if color_idx == 2 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, MAGENTA)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, MAGENTA));
                }
                else if color_idx == 3 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, PURPLE)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, PURPLE));
                }
                else if color_idx == 4 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, BLUE)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, BLUE));
                }
                else if color_idx == 5 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, CYAN)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, CYAN));
                }
                else if color_idx == 6 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, TEAL)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, TEAL));
                }
                else if color_idx == 7 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, GREEN)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, GREEN));
                }
                else if color_idx == 8 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, YELLOW)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, YELLOW));
                }
                else if color_idx == 9 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {TriangleMarker::new(point, 10, YELLOW)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| TriangleMarker::new((x, y), 10, ORANGE));
                }
            } else if switch == 1 {
                if color_idx == 0 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, RED)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, RED));
                }
                else if color_idx == 1 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, PINK)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, PINK));
                }
                else if color_idx == 2 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, MAGENTA)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, MAGENTA));
                }
                else if color_idx == 3 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, PURPLE)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, PURPLE));
                }
                else if color_idx == 4 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, BLUE)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, BLUE));
                }
                else if color_idx == 5 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, CYAN)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, CYAN));
                }
                else if color_idx == 6 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, TEAL)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, TEAL));
                }
                else if color_idx == 7 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, GREEN)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, GREEN));
                }
                else if color_idx == 8 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, YELLOW)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, YELLOW));
                }
                else if color_idx == 9 {
                    ctx.draw_series(
                        points.into_iter().map(|point| {Circle::new(point, 10, YELLOW)})
                    )?
                    .label(&name)
                    .legend(|(x,y)| Circle::new((x, y), 10, ORANGE));
                }
            }
            color_idx += 1;
        }
        ctx.configure_series_labels().border_style(BLACK).draw()?;
        Ok(())
    }
}

impl Export for Entry {
    fn generate_png(&self, start_date: Date, end_date: Date) -> anyhow::Result<()> {
        let dates = crate::utils::expand_dates(&start_date, &end_date);
        let dates_to_usize: HashMap<Date, usize> = {
            let mut out = HashMap::new();
            for (idx, date) in dates.clone().into_iter().enumerate() {
                out.insert(date, idx+1);
            }
            out
        };

        let root_area = BitMapBackend::new("export.png", (1920, 1080))
            .into_drawing_area();
        root_area.fill(&WHITE)?;

        let mut ctx = ChartBuilder::on(&root_area)
            .margin_top(30)
            .margin_right(30)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .build_cartesian_2d(0..dates.len()+1, 0.0_f64..12.0_f64)?;

        ctx
            .configure_mesh()
            .draw()?;

        ctx.draw_series(
            self.get_points(&dates_to_usize).into_iter().map(|point| {TriangleMarker::new(point, 10, &BLACK)})
        )?
        .label(&self.names[0])
        .legend(|(x,y)| TriangleMarker::new((x, y), 10, BLACK));

        ctx.configure_series_labels().border_style(BLACK).draw()?;

        Ok(())
    }
}

