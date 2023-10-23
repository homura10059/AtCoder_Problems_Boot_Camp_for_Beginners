// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::max;

fn acgt_len(s: &str) -> usize {
    if s.len() == 0 {
        return 0;
    }

    let acgt = vec!['A', 'C', 'G', 'T'];
    if s.chars().all(|c| acgt.contains(&c)) {
        return s.len();
    }

    max(acgt_len(&s[1..]), acgt_len(&s[..s.len() - 1]))
}

fn main() {
    input! {
       s: String,
    }

    println!("{}", acgt_len(&s));
}
