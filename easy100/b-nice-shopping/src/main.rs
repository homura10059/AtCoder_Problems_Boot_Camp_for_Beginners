// -*- coding:utf-8-unix -*-

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (a, b, m): (usize, usize, usize),
         a_array: [usize; a],
         b_array: [usize; b],
         xyc: [(usize, usize, usize); m],
    }
    let res = a_array
        .iter()
        .enumerate()
        .cartesian_product(b_array.iter().enumerate())
        .map(|((i, ai), (j, bj))| {
            let c = xyc
                .iter()
                .filter(|(x, y, _)| *x == i + 1 && *y == j + 1)
                .map(|(_, _, c)| c)
                .max()
                .unwrap_or(&0);
            ai + bj - c
        })
        .min()
        .unwrap();
    println!("{}", res);
}
