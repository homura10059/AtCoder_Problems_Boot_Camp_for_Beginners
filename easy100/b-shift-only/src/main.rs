// -*- coding:utf-8-unix -*-

use proconio::input;

fn count_shift(a: Vec<usize>, count: usize) -> usize {
    if a.iter().all(|x| x % 2 == 0) {
        count_shift(a.iter().map(|x| x / 2).collect(), count + 1)
    } else {
        count
    }
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    println!("{}", count_shift(a, 0));
}
