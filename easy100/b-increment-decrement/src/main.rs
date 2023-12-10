// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let result = s
        .chars()
        .map(|c| match c {
            'I' => 1,
            'D' => -1,
            _ => 0,
        })
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .max()
        .unwrap();

    println!("{}", if result < 0 { 0 } else { result });
}
