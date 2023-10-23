// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [usize; n],
    }
    v.sort();
    let result = v.iter().fold(0.0, |acc, x| {
        if acc == 0.0 {
            *x as f64
        } else {
            (acc + *x as f64) / 2.0
        }
    });

    println!("{}", result);
}
