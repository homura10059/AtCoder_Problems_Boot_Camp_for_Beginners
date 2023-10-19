// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        (n, k): (u64, u64)
    }
    println!("{}", min(n % k, k - n % k));
}
