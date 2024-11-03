// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (n, _l): (usize,usize),
        mut s: [String; n],
    }
    s.sort();

    let result = s.join("");
    println!("{}", result);
}
