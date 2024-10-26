// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let rounded = x / 100;
    let fraction = x % 100;
    let goods: Vec<usize> = vec![105, 104, 103, 102, 101, 100];
    let (result, total) = goods.iter().fold((x, 0), |(acc, total), &g| {
        println!("acc: {}, total: {}, g: {}", acc, total, g);
        (acc % g, total + acc / g)
    });

    println!("{}", if result == 0 { "1" } else { "0" });
}
