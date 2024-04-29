// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut b: [usize; n-1],
    }
    b.extend(vec![1000000]);

    let ans = b
        .iter()
        .scan(b[0], |acc, &x| {
            let tmp = x.min(*acc);
            *acc = x; // B_{i-1} を保持しておく
            Some(tmp)
        })
        // .inspect(|x| println!("{x}"))
        .sum::<usize>();
    println!("{}", ans);
}
