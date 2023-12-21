// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
         gates: [(usize, usize); m],
    }

    let (min_l, min_r) = gates.iter().fold((1, n), |(acc_l, acc_r), (l, r)| {
        (acc_l.max(*l), acc_r.min(*r))
    });

    let result = if min_l <= min_r { min_r - min_l + 1 } else { 0 };

    println!("{}", result);
}
