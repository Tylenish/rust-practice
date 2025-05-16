use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut result: Vec<u32> = (0..n).map(|_| rng.gen_range(1..10)).collect();
    let total: u32 = result.iter().sum();
    let remainder = total % n as u32;

    if remainder != 0 {
        result[0] += n as u32 - remainder;
    }

    result
}

fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;

    if total % len != 0 {
        panic!("Неможливо порівну розподілити вантаж між кораблями!");
    }

    let avg = total / len;
    let mut moves = 0;
    let mut diff = 0;

    for &s in shipments {
        diff += s as i32 - avg as i32;
        moves += diff.abs() as usize;
    }

    moves
}

fn count_permutation_safe(shipments: &Vec<u32>) -> Result<usize, String> {
    let total: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;

    if total % len != 0 {
        return Err("Неможливо порівну розподілити вантаж".to_string());
    }

    let avg = total / len;
    let mut moves = 0;
    let mut diff = 0;

    for &s in shipments {
        diff += s as i32 - avg as i32;
        moves += diff.abs() as usize;
    }

    Ok(moves)
}

fn main() {
    // Приклад 1
    let example1 = vec![8, 2, 2, 4, 4];
    println!("Приклад 1: {:?}", example1);
    println!("Очікувана середня: {}", example1.iter().sum::<u32>() / example1.len() as u32);
    println!("Потрібно переміщень: {}", count_permutation(&example1));
    println!("--------------------------------");

    // Приклад 2
    let example2 = vec![9, 3, 7, 2, 9];
    println!("Приклад 2: {:?}", example2);
    println!("Очікувана середня: {}", example2.iter().sum::<u32>() / example2.len() as u32);
    println!("Потрібно переміщень: {}", count_permutation(&example2));
    println!("--------------------------------");

    // Приклад 3 – автоматично згенерований правильний вектор
    let auto = gen_shipments(7);
    println!("Автоматично згенеровано: {:?}", auto);
    match count_permutation_safe(&auto) {
        Ok(moves) => println!("Потрібно переміщень: {}", moves),
        Err(e) => println!("Помилка: {}", e),
    }
}
