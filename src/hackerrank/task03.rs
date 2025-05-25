//https://www.hackerrank.com/challenges/compare-the-triplets/problem?isFullScreen=true
use std::io;

fn compare_triplets(a: Vec<i32>, b: Vec<i32>) -> (i32, i32) {
    let mut alice_score = 0;
    let mut bob_score = 0;

    for i in 0..3 {
        if a[i] > b[i] {
            alice_score += 1;
        } else if a[i] < b[i] {
            bob_score += 1;
        }
    }

    (alice_score, bob_score)
}

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    // Зчитування вводу
    io::stdin().read_line(&mut input1).expect("Не вдалося прочитати рядок");
    io::stdin().read_line(&mut input2).expect("Не вдалося прочитати рядок");

    let a: Vec<i32> = input1
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let b: Vec<i32> = input2
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let (alice_score, bob_score) = compare_triplets(a, b);
    println!("{} {}", alice_score, bob_score);
}
