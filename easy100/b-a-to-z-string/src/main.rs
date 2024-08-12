// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let a = s.find('A').unwrap();
    let z = s.rfind('Z').unwrap();
    println!("{}", z - a + 1);
}
