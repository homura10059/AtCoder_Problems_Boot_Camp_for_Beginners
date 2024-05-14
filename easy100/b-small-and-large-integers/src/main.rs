// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a, b, k): (usize,usize,usize),
    }
    for i in a..(b + 1) {
        if i < a + k || i > b - k {
            println!("{}", i);
        }
    }
}
