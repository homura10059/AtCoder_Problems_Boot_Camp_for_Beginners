// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort_by(|a, b| b.cmp(a));
    let result = a
        .iter()
        .enumerate()
        .map(|(i, x)| if i % 2 == 0 { *x } else { -*x })
        .sum::<i32>();

    println!("{}", result);
}
