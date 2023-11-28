// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let char_set = s.chars().collect::<std::collections::HashSet<char>>();
    let yes = char_set.len() == s.len();
    println!("{}", if yes { "yes" } else { "no" });
}
