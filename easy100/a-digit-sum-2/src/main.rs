// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    }
    let nums = n
        .iter()
        .map(|&c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut result = (0..n.len())
        .map(|i| {
            let num = if nums[i] == 0 { 0 } else { nums[i] - 1 };
            let nine_sum = 9 * (n.len() - (i + 1)) as u32;

            nine_sum + num
        })
        .collect::<Vec<u32>>();
    result.push(nums.iter().sum());

    println!("{}", result.iter().max().unwrap());
}
