use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32
    }

    let result = calc(a, b);
    println!("{}", result)
}

fn calc(sockets: i32, total: i32) -> i32 {
    // すでに一つ口はあるので total - 1 個口を増やしたいという問題
    // 1つ増やすと sockets - 1 個口が増える
    let a = sockets as f32;
    let b = total as f32;
    ((b - 1.0) / (a - 1.0)).ceil() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(a = {
    4, 8, 8, 2
    }, b = {
    10, 9, 8, 20
    }, expected = {
    3, 2, 1, 19
    })]
    fn test_add5(a: i32, b: i32, expected: i32) {
        assert_eq!(calc(a, b), expected);
    }

    #[test]
    fn calc_case1() {
        assert_eq!(calc(2, 20), 19);
    }
}
