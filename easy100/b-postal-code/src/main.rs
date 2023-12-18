// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a, _b): (usize, usize),
        s: String,
    }
    let result = s
        .chars()
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            if i == a && c == '-' || i != a && c != '-' {
                return false;
            }
            true
        })
        .fold(true, |acc, x| acc && x);
    println!("{}", if result { "Yes" } else { "No" });
}
