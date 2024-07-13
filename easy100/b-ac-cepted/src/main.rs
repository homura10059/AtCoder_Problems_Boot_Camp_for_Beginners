// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        s: String,
    }
    let target_index = max(s.len() - 2, 2);
    let (a, c, small) =
        s.chars()
            .enumerate()
            .fold((false, 0, true), |(a, c_count, small), (i, char)| {
                return if i == 0 && char == 'A' {
                    (true, c_count, small)
                } else if i >= 2 && i <= target_index && char == 'C' {
                    (a, c_count + 1, small)
                } else {
                    (a, c_count, small && char.is_ascii_lowercase())
                };
            });

    println!("{}", if a && c == 1 && small { "AC" } else { "WA" });
}
