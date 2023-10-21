// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut tuples: Vec<(String, &usize)> = a
        .iter()
        .enumerate()
        .map(|(i, v)| ((i + 1).to_string(), v))
        .collect();

    tuples.sort_by(|a, b| a.1.cmp(b.1));

    let result = tuples
        .iter()
        .map(|(i, _)| i)
        .cloned()
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", result);
}
