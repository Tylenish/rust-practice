//https://www.hackerrank.com/challenges/staircase/problem?isFullScreen=true
use std::io;

fn staircase(n: usize) {
    for i in 1..=n {
        let spaces = " ".repeat(n - i);
        let hashes = "#".repeat(i);
        println!("{}{}", spaces, hashes);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");
    let n: usize = input.trim().parse().expect("Not a number");

    staircase(n);
}
