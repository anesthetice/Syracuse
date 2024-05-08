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

    let superpoints: Vec<(String, Vec<(f64, f64)>)> = entries.iter().map(|entry| (entry.name.clone(), entry.get_points(&dates))).collect();
    let mut sum_points: Vec<(f64, f64)> = superpoints[0].1.clone();
    for (_, points) in superpoints.iter().skip(1) {
        for (idx, point) in points.into_iter().enumerate() {
            sum_points[idx].1 += point.1
        }
    }
    println!("{:?}", sum_points);
    let sum_points = numerical::interpolate(sum_points);

    let image_width: u32 = dates.len() as u32 * 100 + 6000;
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
        //.x_labels(100)
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

    ctx.draw_series(
        sum_points
            .into_iter()
            .map(|point| Pixel::new(point, fg_rgb)),
    )?;



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

mod numerical {
    use itertools::Itertools;
    use super::linalg;

    // cubic spline interpolation
    pub fn interpolate(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
        let n = points.len() - 1;
        let mut equations = vec![vec![0_f64; 4*n+1]; 4*n];
        let b_idx = 4*n;
        
        // first 2n equations
        for i in 0..n {
            let adj_i = i * 4;
            let x = points[i].0;
            let x_1 = points[i+1].0;

            equations[adj_i][adj_i] = 1.0;      // a0
            equations[adj_i][adj_i+1] = x;      // a1
            equations[adj_i][adj_i+2] = x*x;    // a2
            equations[adj_i][adj_i+3] = x*x*x;  // a3
            equations[adj_i][b_idx] = points[i].1;

            equations[adj_i+1][adj_i] = 1.0;            // a0
            equations[adj_i+1][adj_i+1] = x_1;          // a1
            equations[adj_i+1][adj_i+2] = x_1*x_1;      // a2
            equations[adj_i+1][adj_i+3] = x_1*x_1*x_1;  // a3
            equations[adj_i+1][b_idx] = points[i+1].1;
        }

        // 2(n-1) equations
        for i in 0..n-1 {
            let adj_i = i * 4;
            let adj_i_1 = adj_i + 4;
            let x_1 = points[i+1].0;

            // first derivative
            equations[adj_i+2][adj_i+1] = 1.0;          // a1
            equations[adj_i+2][adj_i+2] = 2.0*x_1;    // a2
            equations[adj_i+2][adj_i+3] = 3.0*x_1*x_1;  // a3

            equations[adj_i+2][adj_i_1+1] = -1.0;          // a1
            equations[adj_i+2][adj_i_1+2] = -2.0*x_1;    // a2
            equations[adj_i+2][adj_i_1+3] = -3.0*x_1*x_1;  // a3

            // second derivative
            equations[adj_i+3][adj_i+2] = 2.0;    // a2
            equations[adj_i+3][adj_i+3] = 6.0*x_1;  // a3

            equations[adj_i+3][adj_i_1+2] = -2.0;    // a2
            equations[adj_i+3][adj_i_1+3] = -6.0*x_1;  // a3
        }

        // 2 equations
        equations[(n-1)*4+2][2] = 2.0;
        equations[(n-1)*4+2][3] = points[0].0 * 3.0;

        equations[(n-1)*4+3][(n-1)*4+2] = 2.0;
        equations[(n-1)*4+3][(n-1)*4+3] = points[n-1].0 * 3.0;


        let solution = {
            if n < 150 {
                linalg::solve_no_para(equations)
            } else {
                linalg::solve_semi_para(equations)
            }
        };

        let splines: Vec<Box<dyn Fn(f64) -> f64>> = solution
            .into_iter()
            .tuple_windows()
            .step_by(4)
            .map(|(a0, a1, a2, a3)| {
                Box::new(move |x: f64| -> f64 {
                    a3 * x*x*x + a2 * x*x + a1 * x + a0
                }) as Box<dyn Fn(f64) -> f64>
            })
            .collect();
        
        let nb_gap = crate::config::Config::get().nb_points_between_dates;
        let dist = (points[0].0 - points[1].0).abs();
        let x_0 = points[0].0.floor() as usize;
        let x_n = points[n].0.floor() as usize;

        (x_0..x_n*nb_gap).map(|x| {
            let x = x as f64 / nb_gap as f64;
            let index = (x / dist).floor() as usize;
            (x, splines.get(index).unwrap_or(splines.last().unwrap())(x))
        }).collect_vec()
    }
    
}


/// tl;dr use solve_no_para if n < 600 and solve_semi_para otherwise
/// n < 600 implies < 150+1 points to interpolate
mod linalg {
    use rayon::iter::{
        IntoParallelRefMutIterator,
        IndexedParallelIterator,
        ParallelIterator,
    };
    type Matrix = Vec<Vec<f64>>;
    type Array = Vec<f64>;

    #[allow(non_snake_case)]
    pub fn solve_no_para(mut Ab: Matrix) -> Array {
        let n: usize = Ab.len();
        let mut r: usize = 0;
        let mut max_r: f64 = f64::MIN;

        for k in 0..n-1 {
            for i in k..n-1 {
                if max_r < Ab[i][k].abs() {
                    r = i;
                    max_r = Ab[i][k].abs()
                }
            }
            Ab.swap(k, r);
            max_r = f64::MIN;

            for i in k+1..n {
                let l_ik = Ab[i][k]/Ab[k][k];
                for j in k..n+1 {
                    Ab[i][j] -= l_ik * Ab[k][j];
                }
            }
        }
        let mut solution: Vec<f64> = Vec::with_capacity(n);

        for i in (0..n).rev() {
            let tmp_ = (1.0/Ab[i][i]) * (Ab[i][n] - (i+1..n).map(|j| {solution[n-(j+1)]*Ab[i][j]}).sum::<f64>());
            solution.push(tmp_);
        }
        solution.reverse();
        solution
    }

    #[allow(non_snake_case)]
    pub fn solve_semi_para(mut Ab: Matrix) -> Array {
        let n: usize = Ab.len();
        let mut r: usize = 0;
        let mut max_r: f64 = f64::MIN;

        for k in 0..n-1 {
            for i in k..n-1 {
                if max_r < Ab[i][k].abs() {
                    r = i;
                    max_r = Ab[i][k].abs()
                }
            }
            Ab.swap(k, r);
            max_r = f64::MIN;

            let Ab_k = Ab[k].clone();
            Ab.par_iter_mut().skip(k+1).for_each(|Ab_i| {
                let l_ik = Ab_i[k]/Ab_k[k];
                for j in k..n+1 {
                    Ab_i[j] -= l_ik * Ab_k[j];
                }
            });
        }
        let mut solution: Vec<f64> = Vec::with_capacity(n);

        for i in (0..n).rev() {
            let tmp_ = (1.0/Ab[i][i]) * (Ab[i][n] - (i+1..n).map(|j| {solution[n-(j+1)]*Ab[i][j]}).sum::<f64>());
            solution.push(tmp_);
        }
        solution.reverse();
        solution
    }
}