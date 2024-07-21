// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let (hit, result) = a.iter().fold((false, 0), |(hit, acc), i| {
        let next = acc + 1;
        if next == *i {
            (true, next)
        } else {
            (hit, acc)
        }
    });

    if hit {
        println!("{}", n - result);
    } else {
        println!("{}", -1);
    }
}
