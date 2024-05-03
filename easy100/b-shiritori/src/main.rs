// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    }
    // 重複排除
    let unique_w = w.iter().collect::<HashSet<_>>();
    if w.len() != unique_w.len() {
        println!("No");
        return;
    }

    // しりとりになっているか
    let initial = w[0].chars().next().unwrap();
    let ans = w
        .iter()
        .scan(initial, |prev_last, word| {
            let first = word.chars().next().unwrap();
            let result = first == *prev_last;
            // println!("{prev_last} == {first} -> {result}");
            *prev_last = word.chars().last().unwrap();
            Some(result)
        })
        .all(|yes| yes);

    println!("{}", if ans { "Yes" } else { "No" });
}
