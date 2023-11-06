// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a, b, m): (usize, usize, usize),
         a_array: [usize; a],
         b_array: [usize; b],
         xyc: [(usize, usize, usize); m],
    }
    let min_ab = a_array.iter().min().unwrap() + b_array.iter().min().unwrap();
    let min = xyc
        .iter()
        .map(|(x, y, c)| a_array[x - 1] + b_array[y - 1] - c)
        .chain([min_ab].iter().copied())
        .min()
        .unwrap();
    println!("{}", min);
}
