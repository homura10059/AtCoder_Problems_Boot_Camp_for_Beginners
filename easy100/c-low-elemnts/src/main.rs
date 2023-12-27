// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let (_min, count) = p.iter().fold((0, 0), |(min, count), v| {
        if count == 0 {
            return (*v, count + 1);
        } else if *v <= min {
            (*v, count + 1)
        } else {
            (min, count)
        }
    });

    println!("{}", count);
}
