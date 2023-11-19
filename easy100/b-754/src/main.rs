// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let result = (0..=s.len() - 3)
        .map(|i| {
            let n = &s[i..i + 3].parse::<i32>().unwrap();
            (n - 753).abs()
        })
        .min()
        .unwrap();
    println!("{}", result);
}
