// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, x): (usize, usize),
        mut a: [usize; n],  // Vec<(i32, i32, i32)>
    }
    let last_index = a.len() - 1;

    a.sort();
    let res: usize = a
        .iter()
        .enumerate()
        .scan(0, |acc, (i, &v)| {
            *acc += v;
            if (i != last_index && *acc <= x) || (i == last_index && *acc == x) {
                Some(1)
            } else {
                None
            }
        })
        .sum();

    println!("{:?}", res);
}
