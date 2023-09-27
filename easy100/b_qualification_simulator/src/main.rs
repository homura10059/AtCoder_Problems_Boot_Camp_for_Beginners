use proconio::input;

fn calc(a: i32, b: i32, s: &Vec<char>) -> Vec<bool> {
    let all_students = s.iter().map(|c| State::from(*c)).collect::<Vec<State>>();

    let mut results: Vec<bool> = vec![];
    let total = (a + b) as usize;
    let mut b_count = 1;

    for student in all_students {
        let true_count = results.iter().filter(|r| **r).count();
        if student == State::A && true_count < total {
            results.push(true);
            continue;
        }
        if student == State::B && true_count < total && b_count <= b {
            results.push(true);
            b_count += 1;
            continue;
        }
        results.push(false);
    }

    results
}

#[derive(PartialEq, Copy, Clone)]
enum State {
    A,
    B,
    C,
}

impl From<char> for State {
    fn from(item: char) -> Self {
        if item == 'a' {
            State::A
        } else if item == 'b' {
            State::B
        } else {
            State::C
        }
    }
}

fn main() {
    input! {
        (n, a, b): (i32, i32, i32),
        s: String
    }

    let results = calc(a, b, &s);
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
        let actual = calc(
            2,
            3,
            &vec!['a', 'b', 'c', 'c', 'a', 'b', 'a', 'a', 'b', 'b'],
        );
        assert_eq!(
            actual,
            vec![true, true, false, false, true, true, true, false, false, false]
        );
    }
}
