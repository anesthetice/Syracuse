use itertools::Itertools;
use plotters::prelude::*;

// cubic splines with quadratic ends

pub fn test() {
    numerical::spline_interpolation_testing();
}

mod numerical {
    use itertools::Itertools;

    use crate::data::graph::linalg;

    pub fn spline_interpolation_testing() {
        let f = |x: f64| {x.exp() - 2.0*x*x - 3.0*x + 5.0};
        let points: Vec<(f64, f64)> = (0..6).map(|x| {
            let x = x as f64;
            (x, f(x))
        }).collect();
        println!("{:?}\n", points);
    
        let n = points.len() - 1;
        let b_idx = 4*n;
        let mut equations = vec![vec![0_f64; 4*n+1]; 4*n];

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

        let solution = linalg::solve_no_para(equations);

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
        
        let y: Vec<f64> = (0..500).map(|x| {
            let x: f64 = x as f64 / 100.0;
            splines.get(x.floor() as usize).unwrap()(x)
        }).collect();

        println!("{:?}", y)

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