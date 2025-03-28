fn main() {
    const WIDTH: usize = 6;
    const SYMBOL: char = '*';

    let mut output = String::new();

    for i in 0..(2 * WIDTH - 1) {
        let spaces = if i < WIDTH { WIDTH - 1 - i } else { i - WIDTH + 1 };
        let stars = if i < WIDTH { 2 * i + 1 } else { 2 * (2 * WIDTH - 2 - i) + 1 };

        output.push_str(&" ".repeat(spaces));
        output.push_str(&SYMBOL.to_string().repeat(stars));
        output.push('\n');
    }

    print!("{}", output);
}