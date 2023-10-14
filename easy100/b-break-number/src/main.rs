// -*- coding:utf-8-unix -*-

use proconio::input;

fn count_divide_2(n: i32) -> i32 {
    let mut count = 0;
    let mut n = n;
    while n % 2 == 0 {
        n /= 2;
        count += 1;
    }
    count
}

fn main() {
    input! {
        n: i32,
    }
    let (max_num, _count) = (1..=n)
        .map(|x| (x, count_divide_2(x)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    println!("{}", max_num);
}
