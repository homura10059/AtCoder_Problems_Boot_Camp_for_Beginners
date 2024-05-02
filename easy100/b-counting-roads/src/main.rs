// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        (n, m): (usize,usize),
        roads: [(usize,usize); m],
    }
    let initial_state = (1..=n).map(|x| (x, 0)).collect::<HashMap<_, _>>();

    let city_roads = roads.iter().fold(initial_state, |mut acc, (a, b)| {
        *acc.entry(*a).or_insert(0) += 1;
        *acc.entry(*b).or_insert(0) += 1;
        acc
    });

    let mut result = city_roads.iter().collect::<Vec<_>>();
    result.sort_by(|a, b| a.0.cmp(b.0));
    result.iter().for_each(|(_key, &roads_count)| {
        println!("{}", roads_count);
    });
}
