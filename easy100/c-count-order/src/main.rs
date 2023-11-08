// -*- coding:utf-8-unix -*-

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let original: Vec<Vec<usize>> = (1..=n).permutations(n).sorted().collect_vec();

    let a = original.iter().position(|x| *x == p).unwrap();
    let b = original.iter().position(|x| *x == q).unwrap();

    println!("{}", a.abs_diff(b));
}
