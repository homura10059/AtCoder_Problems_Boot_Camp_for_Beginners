// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        (a, b): (isize, isize),
    }

    if a <= 0 && 0 <= b {
        println!("Zero");
    } else if 0 < a {
        println!("Positive");
    } else {
        // b < 0
        let count = (a..=b).count();
        if count % 2 == 0 {
            println!("Positive");
        } else {
            println!("Negative");
        }
    }
}
