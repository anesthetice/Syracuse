use itertools::Itertools;
use plotters::prelude::*;

// cubic splines with quadratic ends
pub fn spline_interpolation_testing() {
    let f = |x: f64| {x.exp() - 4.0*x*x - 3.0*x + 5.0};
    let points: Vec<(f64, f64)> = (0..11).map(|x| {
        let x = (x as f64) / 2.0;
        (x, f(x))
    }).collect_vec();
    //println!("{:?}", points);

    let mut splines: Vec<Box<dyn Fn(f64) -> f64>> = Vec::new();
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