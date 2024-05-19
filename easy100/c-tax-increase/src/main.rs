// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a,b): (usize,usize),
    }

    (1..=100_000)
        .filter(|&x| x * 8 / 100 == a && x * 10 / 100 == b)
        .next()
        .map_or_else(|| println!("-1"), |x| println!("{}", x));
}
