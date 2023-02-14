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
    let mut num = total / 2;
    let mut result = 0;
    loop {
        let sum = sockets * num - (num - 1);

        if sum >= total {
            result = num;
            num -= 1;
        } else {
            num += 1;
        }

        if result == num {
            break;
        }
    }
    result
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
