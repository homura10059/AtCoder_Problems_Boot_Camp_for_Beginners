// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        w: String,
    }
    let not_beautiful_count = w
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
        .iter()
        .filter(|(_, count)| **count % 2 != 0)
        .count();

    let yes = not_beautiful_count == 0;

    println!("{}", if yes { "Yes" } else { "No" });
}
