// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: [[i32; 3]; 3],
        n: usize,
        b: [i32; n],

    }
    let horizontal = a.clone();
    let vertical = (0..=2)
        .map(|i| vec![horizontal[0][i], horizontal[1][i], horizontal[2][i]])
        .collect::<Vec<Vec<i32>>>();
    let diagonal = vec![
        vec![horizontal[0][0], horizontal[1][1], horizontal[2][2]],
        vec![horizontal[0][2], horizontal[1][1], horizontal[2][0]],
    ];
    let all_nums = [horizontal, vertical, diagonal].concat();
    let yes = all_nums
        .iter()
        .any(|nums| nums.iter().all(|&num| b.contains(&num)));
    println!("{}", if yes { "Yes" } else { "No" });
}
