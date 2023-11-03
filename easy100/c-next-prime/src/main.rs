// -*- coding:utf-8-unix -*-

use proconio::input;

fn sieve_of_eratosthenes(x: usize) -> Vec<usize> {
    let mut num_list = vec![true; x + 1];
    num_list[0] = false;
    num_list[1] = false;

    for i in 2..=(x as f64).sqrt() as usize {
        if !num_list[i] {
            continue;
        }
        for j in (i * 2..=x).step_by(i) {
            num_list[j] = false;
        }
    }

    num_list
        .iter()
        .enumerate()
        .filter(|(_, &flag)| flag)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>()
}

fn main() {
    input! {
        x: usize,
    }
    let prime_list = sieve_of_eratosthenes(x * 2);

    println!("{}", prime_list.iter().find(|&&p| p >= x).unwrap());
}
