// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut nums = a.clone();
    nums.sort();
    let (next_max, max) = (nums[n - 2], nums[n - 1]);
    let max_count = a.iter().filter(|&x| *x == max).count();

    a.iter().for_each(|x| {
        if *x == max {
            if max_count == 1 {
                println!("{}", next_max);
            } else {
                println!("{}", max);
            }
        } else {
            println!("{}", max);
        }
    });
}
