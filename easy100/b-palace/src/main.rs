// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        (t,a): (isize,isize),
        h: [usize; n],
    }

    h.iter()
        .enumerate()
        .min_by_key(|&(_i, &x)| (1000 * t - x as isize * 6).abs_diff(1000 * a))
        .map(|(i, _x)| {
            println!("{}", i + 1);
        });
}
