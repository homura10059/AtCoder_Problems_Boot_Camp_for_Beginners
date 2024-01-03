// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        (n, _m): (usize, usize),
    }
    let mut like_map = HashMap::new();

    (0..n).for_each(|_| {
        input! {
            k: usize,
            a: [usize; k],
        }
        a.iter().for_each(|&i| {
            *like_map.entry(i).or_insert(0) += 1;
        });
    });

    let result = like_map.iter().filter(|(_, &v)| v == n).count();

    println!("{}", result);
}
