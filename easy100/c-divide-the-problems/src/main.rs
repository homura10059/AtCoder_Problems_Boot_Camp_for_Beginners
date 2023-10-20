// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [i32; n],
    }
    d.sort();
    let half = n / 2;
    let a = d[half - 1];
    let b = d[half];
    println!("{}", b - a);
}
