// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a, b, k): (usize,usize,usize),
    }
    (a..=b)
        .filter(|&x| x < a + k || x > b - k)
        .for_each(|x| println!("{}", x));
}
