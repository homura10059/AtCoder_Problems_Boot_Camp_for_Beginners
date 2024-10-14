// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;

fn main() {
    input! {
        s: Chars,
    }
    let n = s.len();

    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 1;
    dp[1][1] = 1;

    for i in 1..n {
        let tmp = dp[i - 1][1];
        dp[i][0] = max(dp[i][0], tmp + 1);

        if s[i - 1] != s[i] {
            let tmp = dp[i - 1][0];
            dp[i][0] = max(dp[i][0], tmp + 1);
        }
        if i >= 2 {
            dp[i][1] = max(dp[i][1], dp[i - 2][0] + 1);
            dp[i][1] = max(dp[i][1], dp[i - 2][1] + 1);
        }
    }
    println!("{}", max(dp[n - 1][0], dp[n - 1][1]));
}
