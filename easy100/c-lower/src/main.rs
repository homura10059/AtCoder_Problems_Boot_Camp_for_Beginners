// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let (_prev, count, max) =
        h.iter()
            .enumerate()
            .fold((0, 0, 0), |(prev, count, max), (i, &h)| {
                // println!("({}, {}, {}), ({}, {})", prev, count, max, i, h);
                if i == 0 {
                    (h, 0, 0)
                } else if prev >= h {
                    (h, count + 1, max)
                } else {
                    (h, 0, count.max(max))
                }
            });

    println!("{}", count.max(max));
}
