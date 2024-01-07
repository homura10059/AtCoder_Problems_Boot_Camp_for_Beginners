// -*- coding:utf-8-unix -*-

use proconio::input;

fn is_palindromic_number(n: usize) -> bool {
    let s = n.to_string();
    let r = s.chars().rev().collect::<String>();
    s == r
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let count = (a..=b).filter(|&n| is_palindromic_number(n)).count();
    println!("{}", count);
}
