//https://www.hackerrank.com/challenges/simple-array-sum/problem

use std::io;

fn simple_array_sum(arr: Vec<i32>) -> i32 {
    arr.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let result = simple_array_sum(arr);
    println!("{}", result);
}
