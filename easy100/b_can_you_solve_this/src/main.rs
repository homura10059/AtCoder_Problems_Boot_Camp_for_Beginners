use proconio::input;

fn sum(a: Vec<i8>, b: Vec<i8>) -> i32 {
    a.iter()
        .zip(b.iter())
        .map(|(ai, bi)| *ai as i32 * *bi as i32)
        .sum()
}

fn calc(a: Vec<Vec<i8>>, b: Vec<i8>, c: i8) -> usize {
    a.iter()
        .map(|ai| sum(ai.clone(), b.clone()) + c as i32)
        .filter(|&x| x > 0)
        .count()
}

fn main() {
    input! {
       (n, m, c): (i8, i8, i8),
        b: [i8; m],
        a: [[i8; m]; n]
    }

    let result = calc(a, b, c);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let c = -10;
        let b = vec![1, 2, 3];
        let a = vec![vec![3, 2, 1], vec![1, 2, 2]];
        let actual = calc(a, b, c);
        assert_eq!(actual, 1);
    }

    #[test]
    fn test_2() {
        let c = 0;
        let b = vec![100, -100, 0];
        let a = vec![vec![0, 100, 100], vec![100, 100, 100], vec![-100, 100, 100]];
        let actual = calc(a, b, c);
        assert_eq!(actual, 0);
    }
}
