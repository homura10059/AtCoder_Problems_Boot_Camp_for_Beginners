// -*- coding:utf-8-unix -*-

use proconio::input;

fn calc_penalty(cook_time: usize) -> usize {
    let remainder = cook_time % 10;
    if remainder == 0 {
        0
    } else {
        10 - remainder
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
    }
    let mut cook_times = vec![a, b, c, d, e]
        .iter()
        .map(|&x| (x, calc_penalty(x)))
        .collect::<Vec<(usize, usize)>>();
    cook_times.sort_by(|(_a, ap), (_b, bp)| ap.cmp(bp));
    let result = cook_times.iter().enumerate().fold(0, |acc, (i, x)| {
        if i == cook_times.len() - 1 {
            acc + x.0
        } else {
            acc + x.0 + x.1
        }
    });

    println!("{}", result);
}
