// -*- coding:utf-8-unix -*-

use proconio::input;

fn get_exponential_nums(base: usize, max: usize) -> Vec<usize> {
    let mut nums = vec![];
    for i in 2..=max {
        let n = base.pow(i as u32);
        if n > max {
            break;
        }
        nums.push(n);
    }
    nums
}

fn main() {
    input! {
        x: usize,
    }
    // 31^2 = 961 なのでそこまで準備しておけば十分
    let result = (1..=35)
        .map(|b| {
            if b == 1 {
                vec![1]
            } else {
                get_exponential_nums(b, x)
            }
        })
        .flatten()
        .max()
        .unwrap();

    println!("{}", result);
}
