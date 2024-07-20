// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (h, w): (usize,usize),
        c: [String; h],  // Vec<(i32, i32, i32)>
    }
    c.iter().for_each(|row| {
        println!("{}", row);
        println!("{}", row);
    });
}
