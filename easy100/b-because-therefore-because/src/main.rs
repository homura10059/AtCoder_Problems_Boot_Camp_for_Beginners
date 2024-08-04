// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        o: String,
        e: String,
    }
    let result = o
        .chars()
        .zip(e.chars())
        .fold(String::new(), |mut acc, (oi, ei)| {
            acc.push(oi);
            acc.push(ei);
            acc
        });

    // zip だと短い方に合わせるので、oが長い場合は別途考慮
    if o.len() > e.len() {
        println!("{}{}", result, o.chars().last().unwrap());
    } else {
        println!("{}", result);
    }
}
