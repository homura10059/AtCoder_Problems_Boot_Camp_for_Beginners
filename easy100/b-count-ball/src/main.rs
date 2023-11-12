// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }
    let div = n / (a + b);
    let rem = n % (a + b);

    println!("{}", div * a + std::cmp::min(rem, a));
}
