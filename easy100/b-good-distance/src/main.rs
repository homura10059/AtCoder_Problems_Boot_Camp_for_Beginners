// -*- coding:utf-8-unix -*-

use proconio::input;

fn distance(x: &Vec<i32>, y: &Vec<i32>) -> f32 {
    let sum = x
        .iter()
        .zip(y.iter())
        .map(|(x, y)| x.abs_diff(*y).pow(2))
        .sum::<u32>() as f32;
    sum.sqrt()
}

fn main() {
    input! {
        (n, d): (usize,usize),
        x: [[i32; d]; n],
    }
    let result = x
        .iter()
        .enumerate()
        .flat_map(|(i, xi)| {
            let sub_x = x.iter().skip(i + 1);
            sub_x.map(move |xj| distance(&xi, &xj))
        })
        .filter(|&d| d.fract() == 0.0)
        .count();

    println!("{}", result);
}
