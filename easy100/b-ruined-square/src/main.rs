// -*- coding:utf-8-unix -*-

use ndarray::prelude::*;
use proconio::input;

fn main() {
    input! {
        (x1,y1,x2,y2): (f64, f64, f64, f64)
    }
    let rotation_matrix = arr2(&[[0_f64, -1_f64], [1_f64, 0_f64]]);
    let dx = x2 - x1;
    let dy = y2 - y1;
    let d2 = rotation_matrix.dot(&arr2(&[[dx], [dy]]));
    let d3 = rotation_matrix.dot(&d2);

    let (x3, y3) = (x2 + d2[[0, 0]], y2 + d2[[1, 0]]);
    let (x4, y4) = (x3 + d3[[0, 0]], y3 + d3[[1, 0]]);

    println!("{} {} {} {}", x3, y3, x4, y4);
}
