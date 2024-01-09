// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (a,b,_c,k): (u64, u64, u64, u64),
    }
    let result = a.abs_diff(b);

    if result > 10u64.pow(18) {
        println!("Unfair");
    }

    if k % 2 == 0 {
        println!("{}", a as i64 - b as i64);
    } else {
        println!("{}", b as i64 - a as i64);
    }
}
