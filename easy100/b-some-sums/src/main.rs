// -*- coding:utf-8-unix -*-

use proconio::input;

fn sum_digit(n: usize) -> usize {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum()
}

fn main() {
    input! {
        (n, a,b ): (usize,usize,usize)
    }
    let ans = (0..=n)
        .map(|x| (x, sum_digit(x)))
        .filter(|(_x, sum)| a <= *sum && *sum <= b)
        // .inspect(|(x, sum)| println!("{x}, {sum}"))
        .map(|(x, _sum)| x)
        .sum::<usize>();
    println!("{}", ans);
}
