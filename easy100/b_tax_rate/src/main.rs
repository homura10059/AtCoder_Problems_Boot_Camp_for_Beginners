use proconio::input;

fn calc(n: i32) -> Option<i32> {
    for x in 0..=50000 {
        let in_tax = (x as f32 * 1.08).floor() as i32;
        if in_tax == n {
            return Some(x);
        }
    }
    None
}

fn main() {
    input! {
        n: i32,
    }

    let result = calc(n);
    println!("{}", result.map_or(":(".to_string(), |x| x.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual = calc(432);
        assert_eq!(actual, Some(400));
    }

    #[test]
    fn test_2() {
        let actual = calc(1079);
        assert_eq!(actual, None);
    }

    #[test]
    fn test_3() {
        let actual = calc(1001);
        assert_eq!(actual, Some(927));
    }
}
