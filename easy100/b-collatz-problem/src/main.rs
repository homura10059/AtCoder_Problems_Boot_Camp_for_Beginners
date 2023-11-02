// -*- coding:utf-8-unix -*-

use proconio::input;

fn calc(n: usize) -> usize {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn result(before: Vec<usize>) -> usize {
    let n = before.last().unwrap().clone();
    let a = calc(n);
    if before.contains(&a) {
        return before.len() + 1;
    }
    result([before, vec![a]].concat())
}

fn main() {
    input! {
        s: usize,
    }
    println!("{}", result(vec![s]));
}
