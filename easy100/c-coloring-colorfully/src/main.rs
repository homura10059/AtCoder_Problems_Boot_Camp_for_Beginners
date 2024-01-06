// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    if s.len() == 1 {
        println!("0");
        return;
    }
    let color_array = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let odd_count = color_array
        .iter()
        .enumerate()
        .filter(|(i, c)| (i % 2 == 0 && **c == 0) || (i % 2 == 1 && **c == 1))
        .count();

    let even_count = color_array
        .iter()
        .enumerate()
        .filter(|(i, c)| (i % 2 == 0 && **c == 1) || (i % 2 == 1 && **c == 0))
        .count();

    println!("{}", odd_count.min(even_count));
}
