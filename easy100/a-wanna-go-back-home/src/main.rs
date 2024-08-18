// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let chars = s.chars().collect::<Vec<char>>();
    let (n, s) = (
        chars.iter().find(|&c| c == &'N').is_some(),
        chars.iter().find(|&c| c == &'S').is_some(),
    );
    let (w, e) = (
        chars.iter().find(|&c| c == &'W').is_some(),
        chars.iter().find(|&c| c == &'E').is_some(),
    );

    println!("{}", if !(n ^ s) && !(w ^ e) { "Yes" } else { "No" });
}
