// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
         (a, b): (i32, i32),
    }
    let a_str = a.to_string();
    let b_str = b.to_string();
    let c_str = format!("{}{}", a_str, b_str);
    let c = c_str.parse::<f32>().unwrap();
    let root = c.sqrt().floor();

    println!("{}", if c == root.powi(2) { "Yes" } else { "No" });
}
