//https://www.hackerrank.com/challenges/diagonal-difference/problem?isFullScreen=true
use std::io;

fn diagonal_difference(arr: Vec<Vec<i32>>) -> i32 {
    let n = arr.len();
    let mut primary_diagonal = 0;
    let mut secondary_diagonal = 0;

    for i in 0..n {
        primary_diagonal += arr[i][i];
        secondary_diagonal += arr[i][n - 1 - i];
    }

    (primary_diagonal - secondary_diagonal).abs()
}

fn main() {
    let mut n_line = String::new();
    io::stdin().read_line(&mut n_line).expect("Не вдалося прочитати рядок");
    let n: usize = n_line.trim().parse().expect("Not a number");

    let mut arr: Vec<Vec<i32>> = Vec::new();

    for _ in 0..n {
        let mut row = String::new();
        io::stdin().read_line(&mut row).expect("Не вдалося прочитати рядок");
        let row_vec: Vec<i32> = row
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number"))
            .collect();
        arr.push(row_vec);
    }

    let result = diagonal_difference(arr);
    println!("{}", result);
}
