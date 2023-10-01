use log::info;
use proconio::input;

fn calc(x: &Vec<i16>) -> i32 {
    info!("{:?}", x);
    0
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
        assert_eq!(actual, 0);
    }
}
