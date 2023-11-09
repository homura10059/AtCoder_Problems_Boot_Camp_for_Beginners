// -*- coding:utf-8-unix -*-

use proconio::input;

fn attack(enemy: Vec<usize>) -> usize {
    if enemy.iter().all(|&e| e == 1) {
        return enemy.len();
    }

    let next = enemy
        .iter()
        .map(|&e| if e == 1 { vec![1] } else { vec![e / 2] })
        .flatten()
        .collect();

    attack(next) * 2 + enemy.len()
}

fn main() {
    input! {
        mut h: usize,
    }
    println!("{}", attack(vec![h]));
}
