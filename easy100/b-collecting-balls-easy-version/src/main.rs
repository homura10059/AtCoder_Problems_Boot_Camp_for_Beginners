// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
        x: [i32; n],
    }
    let total = x.iter().fold(0, |sum, &xi| sum + 2 * xi.min(k - xi));
    println!("{}", total);
}
