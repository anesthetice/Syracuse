use itertools::Itertools;
use plotters::prelude::*;

// cubic splines with quadratic ends
fn spline_interpolation_testing() {
    let f = |x: f64| {x.exp() - 3.0*x + 5.0};
    let points: Vec<(f64, f64)> = (0..11).map(|x| {
        let x = (x as f64) / 2.0;
        (x, f(x))
    }).collect_vec();
    println!("{:?}", points);
}