// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        (d, x): (usize, usize),
        a: [usize; n],
    }
    let count = a
        .iter()
        .map(|ai| {
            let days = vec![0; d];

            let all = days
                .iter()
                .enumerate()
                .map(|(i, _)| if i == 0 || i % ai == 0 { 1 } else { 0 })
                .sum::<usize>();

            all
        })
        .sum::<usize>();

    println!("{}", count + x);
}
