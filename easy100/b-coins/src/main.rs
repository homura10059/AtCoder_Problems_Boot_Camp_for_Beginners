// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let target = x / 50;
    let coins = vec![(10, a), (2, b), (1, c)];
    let result = coins
        .iter()
        .map(|(unit, n)| (0..=*n).map(|i| i * unit).collect::<Vec<usize>>())
        .fold(vec![0], |acc, v| {
            acc.iter()
                .flat_map(|&i| v.iter().map(move |j| j + i))
                .collect::<Vec<usize>>()
        })
        .iter()
        .filter(|&&i| i == target)
        .count();

    println!("{}", result);
}
