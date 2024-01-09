// -*- coding:utf-8-unix -*-
use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let lucas_numbers = (2..=n).fold(vec![2_u64, 1_u64], |mut acc, _| {
        let len = acc.len();
        acc.push(acc[len - 1] + acc[len - 2]);
        acc
    });

    let result = lucas_numbers.last().unwrap();

    println!("{}", result);
}
