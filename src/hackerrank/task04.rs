//https://www.hackerrank.com/challenges/a-very-big-sum/problem?isFullScreen=true
use std::io;

fn a_very_big_sum(ar: Vec<i64>) -> i64 {
    ar.iter().sum()
}

fn main() {
    let mut n_line = String::new();
    let mut ar_line = String::new();

    // Зчитуємо кількість елементів (n), але вона не використовується безпосередньо
    io::stdin().read_line(&mut n_line).expect("Не вдалося прочитати рядок");

    // Зчитуємо сам масив
    io::stdin().read_line(&mut ar_line).expect("Не вдалося прочитати рядок");

    let ar: Vec<i64> = ar_line
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let result = a_very_big_sum(ar);
    println!("{}", result);
}
