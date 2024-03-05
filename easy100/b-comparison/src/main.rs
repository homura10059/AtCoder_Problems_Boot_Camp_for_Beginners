// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }
    if a == b {
        println!("EQUAL");
    } else if a.len() > b.len() {
        println!("GREATER");
    } else if a.len() < b.len() {
        println!("LESS");
    } else {
        // 桁数が同じなので、Vecの長さも同じはず
        let char_tuple: Vec<(char, char)> = a.chars().zip(b.chars()).collect();
        for (a, b) in char_tuple {
            if a > b {
                println!("GREATER");
                break;
            } else if a < b {
                println!("LESS");
                break;
            }
        }
    }
}
