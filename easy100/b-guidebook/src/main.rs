// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        sp: [(String, usize); n],
    }
    let mut result = sp
        .iter()
        .enumerate()
        .map(|(i, (s, p))| (i + 1, s.clone(), *p))
        .collect::<Vec<(usize, String, usize)>>();

    result.sort_by(|(_, a_str, a_p), (_, b_str, b_p)| {
        if a_str != b_str {
            a_str.cmp(b_str)
        } else {
            // Sort in descending order
            b_p.cmp(a_p)
        }
    });

    result.iter().for_each(|(i, _, _)| println!("{}", i));
}
