// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize; n],
        m: usize,
        p: [(usize, i32); m],
    }
    let first_sum: usize = t.iter().sum();
    p.iter().for_each(|(i, x)| {
        let ti = t[*i - 1];
        let diff = ti as isize - *x as isize;
        let total = first_sum as isize - diff;
        println!("{}", total);
    });
}
