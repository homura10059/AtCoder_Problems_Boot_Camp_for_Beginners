// -*- coding:utf-8-unix -*-

use proconio::input;

fn count_to(a: i64, b: i64, c: i64, count: i64) -> i64 {
    if a % 2 == 1 || b % 2 == 1 || c % 2 == 1 {
        return count;
    }
    let half_a = a / 2;
    let half_b = b / 2;
    let half_c = c / 2;
    let next_a = half_b + half_c;
    let next_b = half_a + half_c;
    let next_c = half_a + half_b;
    count_to(next_a, next_b, next_c, count + 1)
}

fn main() {
    input! {
        (a, b, c): (i64, i64, i64)
    }
    if a % 2 != 1 && b % 2 != 1 && c % 2 != 1 && a == b && b == c {
        println!("-1");
        return;
    }

    println!("{}", count_to(a, b, c, 0));
}
