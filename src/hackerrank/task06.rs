//https://www.hackerrank.com/challenges/plus-minus/problem?isFullScreen=true
use std::io;

fn plus_minus(arr: Vec<i32>) {
    let n = arr.len() as f64;
    let mut positive = 0.0;
    let mut negative = 0.0;
    let mut zero = 0.0;

    for &num in &arr {
        if num > 0 {
            positive += 1.0;
        } else if num < 0 {
            negative += 1.0;
        } else {
            zero += 1.0;
        }
    }

    println!("{:.6}", positive / n);
    println!("{:.6}", negative / n);
    println!("{:.6}", zero / n);
}

fn main() {
    let mut n_line = String::new();
    let mut arr_line = String::new();

    io::stdin().read_line(&mut n_line).expect("Не вдалося прочитати рядок");
    io::stdin().read_line(&mut arr_line).expect("Не вдалося прочитати рядок");

    let arr: Vec<i32> = arr_line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    plus_minus(arr);
}

