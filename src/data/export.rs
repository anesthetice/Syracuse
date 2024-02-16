use plotters::{prelude::*, style::full_palette::{ORANGE, PINK, PURPLE, TEAL}};
use std::collections::HashMap;
use time::Date;

use super::internal::{Entries, Entry};

const COLORS: [RGBColor; 10] = [RED, PINK, MAGENTA, PURPLE, BLUE, CYAN, TEAL, GREEN, YELLOW, ORANGE];

pub trait Export {
    fn generate_png(&self, num_of_days_back: u16, end_date: Date) -> anyhow::Result<()>;
}

macro_rules! draw_triangle_series {
    ($ctx:expr, $points:expr, $name:expr, $color:expr) => {
        $ctx.draw_series(
            $points.into_iter().map(|point| TriangleMarker::new(point, 10, $color))
        )?
        .label($name)
        .legend(|(x, y)| TriangleMarker::new((x, y), 10, $color));
    };
}

macro_rules! draw_circle_series {
    ($ctx:expr, $points:expr, $name:expr, $color:expr) => {
        $ctx.draw_series(
            $points.into_iter().map(|point| TriangleMarker::new(point, 10, $color))
        )?
        .label($name)
        .legend(|(x, y)| Circle::new((x, y), 10, $color));
    };
}

impl Export for Entries {
    fn generate_png(&self, num_of_days_back: u16, end_date: Date) -> anyhow::Result<()> {
        let dates = crate::utils::expand_date_backwards(num_of_days_back, &end_date);
        println!("{:?}", dates);
        let dates_to_usize: HashMap<Date, usize> = {
            let mut out = HashMap::new();
            for (idx, date) in dates.clone().into_iter().enumerate() {
                out.insert(date, idx);
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

        println!("{:?}", &points_list);

        let mut color_idx: usize = 0; let max_color_idx = COLORS.len(); let mut switch: u8 = 0;

        while let Some((name, points)) = points_list.pop() {
            if color_idx == max_color_idx {
                color_idx = 0;
                switch += 1;
            }
            // as "nice" as I can make it, plotters still needs a lot of work...
            if switch == 0 {
                if color_idx == 0 {
                    draw_triangle_series!(ctx, points, name, RED);
                }
                else if color_idx == 1 {
                    draw_triangle_series!(ctx, points, name, PINK);
                }
                else if color_idx == 2 {
                    draw_triangle_series!(ctx, points, name, MAGENTA);
                }
                else if color_idx == 3 {
                    draw_triangle_series!(ctx, points, name, PURPLE);
                }
                else if color_idx == 4 {
                    draw_triangle_series!(ctx, points, name, BLUE);
                }
                else if color_idx == 5 {
                    draw_triangle_series!(ctx, points, name, CYAN);
                }
                else if color_idx == 6 {
                    draw_triangle_series!(ctx, points, name, TEAL);
                }
                else if color_idx == 7 {
                    draw_triangle_series!(ctx, points, name, GREEN);
                }
                else if color_idx == 8 {
                    draw_triangle_series!(ctx, points, name, YELLOW);
                }
                else if color_idx == 9 {
                    draw_triangle_series!(ctx, points, name, ORANGE);
                }
            } else if switch == 1 {
                if color_idx == 0 {
                    draw_circle_series!(ctx, points, name, RED);
                }
                else if color_idx == 1 {
                    draw_circle_series!(ctx, points, name, PINK);
                }
                else if color_idx == 2 {
                    draw_circle_series!(ctx, points, name, MAGENTA);
                }
                else if color_idx == 3 {
                    draw_circle_series!(ctx, points, name, PURPLE);
                }
                else if color_idx == 4 {
                    draw_circle_series!(ctx, points, name, BLUE);
                }
                else if color_idx == 5 {
                    draw_circle_series!(ctx, points, name, CYAN);
                }
                else if color_idx == 6 {
                    draw_circle_series!(ctx, points, name, TEAL);
                }
                else if color_idx == 7 {
                    draw_circle_series!(ctx, points, name, GREEN);
                }
                else if color_idx == 8 {
                    draw_circle_series!(ctx, points, name, YELLOW);
                }
                else if color_idx == 9 {
                    draw_circle_series!(ctx, points, name, ORANGE);
                }
            }
            color_idx += 1;
        }
        ctx.configure_series_labels().border_style(BLACK).draw()?;
        Ok(())
    }
}

impl Export for Entry {
    fn generate_png(&self, num_of_days_back: u16, end_date: Date) -> anyhow::Result<()> {
        let dates = crate::utils::expand_date_backwards(num_of_days_back, &end_date);
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

        draw_triangle_series!(ctx, self.get_points(&dates_to_usize), self.names.get(0).unwrap_or(&"None".to_string()).to_string(), BLACK);

        ctx.configure_series_labels().border_style(BLACK).draw()?;

        Ok(())
    }
}

