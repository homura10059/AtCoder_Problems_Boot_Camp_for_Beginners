// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        k: i32,
        n: i32,
        mut a: [i32; n]
    }
    let first = a.first().unwrap();
    a.push(*first);
    let mut distances = a
        .windows(2)
        .map(|x| {
            let (x1, x2) = (x[0], x[1]);
            if x1 > x2 {
                k - x1 + x2
            } else {
                x2 - x1
            }
        })
        .collect::<Vec<i32>>();
    distances.sort();

    let dist = distances
        .iter()
        .take((n - 1) as usize)
        // .inspect(|x| println!("{:?}", x))
        .sum::<i32>();

    println!("{}", dist);
}
