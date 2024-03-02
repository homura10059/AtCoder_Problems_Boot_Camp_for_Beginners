// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [String; h],
    }
    let start_end = (0..w + 2).map(|_| "#").collect::<String>();

    println!("{}", start_end);
    a.iter().for_each(|row| {
        println!("#{}#", row);
    });
    println!("{}", start_end);
}
