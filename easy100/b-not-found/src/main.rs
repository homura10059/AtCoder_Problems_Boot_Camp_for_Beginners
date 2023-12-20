// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: String
    }
    let unique_chars = s.chars().collect::<HashSet<char>>();

    let alphabet = "abcdefghijklmnopqrstuvwxyz".to_string();
    let mut not_found_chars = alphabet
        .chars()
        .filter(|c| !unique_chars.contains(c))
        .collect::<Vec<char>>();
    not_found_chars.sort();

    let result = not_found_chars
        .get(0)
        .map_or("None".to_string(), |c| c.to_string());

    println!("{}", result);
}
