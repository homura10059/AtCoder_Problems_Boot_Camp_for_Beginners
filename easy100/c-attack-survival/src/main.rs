// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        (n, k, q): (usize, usize, usize),
        a: [usize; q],
    }
    let last_point_for_all = k as isize - q as isize;
    let correct_count_map = a.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    for ni in 1..=n {
        let correct_count = correct_count_map.get(&ni).unwrap_or(&0);
        if last_point_for_all + correct_count > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
