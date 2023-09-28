use proconio::input;

fn calc(a: i32, b: i32, s: String) -> Vec<bool> {
    let mut results: Vec<bool> = vec![];
    let total = (a + b) as usize;
    let mut b_count = 1;
    let mut true_count = 0;

    for student in s.chars() {
        if student == 'a' && true_count < total {
            results.push(true);
            true_count += 1;
            continue;
        }
        if student == 'b' && true_count < total && b_count <= b {
            results.push(true);
            true_count += 1;
            b_count += 1;
            continue;
        }
        results.push(false);
    }

    results
}

fn main() {
    input! {
        (n, a, b): (i32, i32, i32),
        s: String
    }

    let results = calc(a, b, s);
    for result in results {
        let r = if result { "Yes" } else { "No" };
        println!("{}", r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let actual = calc(2, 3, "abccabaabb".to_string());
        assert_eq!(
            actual,
            vec![true, true, false, false, true, true, true, false, false, false]
        );
    }
}
