// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        o: String,
        e: String,
    }
    let result = o
        .chars()
        .zip(e.chars())
        .fold(String::new(), |mut acc, (oi, ei)| {
            acc.push(oi);
            acc.push(ei);
            acc
        });

    print!("{}", result);
}
