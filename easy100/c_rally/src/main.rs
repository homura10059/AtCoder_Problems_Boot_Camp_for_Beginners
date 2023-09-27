use proconio::input;

fn cost(x: i16, p: i16) -> i32 {
    (x - p).pow(2) as i32
}

fn all_costs(x: &Vec<i16>, p: i16) -> i32 {
    x.iter().map(|i| cost(*i, p)).sum()
}

fn calc(x: &Vec<i16>) -> i32 {
    let min = *x.iter().min().unwrap();
    let max = *x.iter().max().unwrap();

    // let all_points = 0..101;
    let all_points = min..=max;

    let hash_map = all_points
        .into_iter()
        .map(|p| (p, all_costs(x, p)))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    hash_map.1
}

fn main() {
    input! {
        n: i16,
        x: [i16; n]
    }

    let result = calc(&x);
    println!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual = calc(&vec![1, 4]);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_2() {
        let actual = calc(&vec![14, 14, 2, 13, 56, 2, 37]);
        assert_eq!(actual, 2354);
    }
}
