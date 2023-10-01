use proconio::input;

fn calc((h, w): (i32, i32)) -> i64 {
    let mul = h as i64 * w as i64;
    if mul % 2 == 0 {
        mul / 2
    } else {
        (mul + 1) / 2
    }
}

fn main() {
    input! {
        (h,w): (i32, i32),
    }

    let result = calc((h, w));
    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual = calc((4, 5));
        assert_eq!(actual, 10);
    }

    #[test]
    fn test_2() {
        let actual = calc((7, 3));
        assert_eq!(actual, 11);
    }

    #[test]
    fn test_3() {
        let actual = calc((1000000000, 1000000000));
        assert_eq!(actual, 500000000000000000);
    }

    #[test]
    fn test_4() {
        let actual = calc((1, 1));
        assert_eq!(actual, 1);
    }
}
