// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let map = s
        .iter()
        .fold(HashMap::<String, usize>::new(), |mut map, s| {
            map.entry(s.clone()).and_modify(|e| *e += 1).or_insert(1);
            map
        });
    let max = map.values().max().unwrap();
    let mut result = map
        .iter()
        .filter(|(_, v)| *v == max)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    result.sort();
    for r in result {
        println!("{}", r);
    }
}
