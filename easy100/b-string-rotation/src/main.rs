// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }
    let (result, _) = (0..s.len()).fold((false, s), |(result, str), _i| {
        if str == t {
            (true, str)
        } else {
            let str = format!("{}{}", &str[1..], &str[0..1]);
            (result, str)
        }
    });

    println!("{}", if result { "Yes" } else { "No" });
}
