// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, m, x): (usize, usize, usize),
        a: [usize; m],
    }
    let mut row = vec![0; n];
    a.iter().for_each(|&i| row[i] = 1);

    let left = row[..x].iter().sum::<usize>();
    let right = row[x..].iter().sum::<usize>();

    println!("{}", left.min(right));
}
