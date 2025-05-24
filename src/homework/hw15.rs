pub fn solve() {
    use itertools::Itertools;

    let mut count = 0;

    for perm in (1..=8).permutations(8) {
        let (m, u, x, a, s, l, o, n) = (
            perm[0], perm[1], perm[2], perm[3],
            perm[4], perm[5], perm[6], perm[7],
        );

        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muxa * a == slon {
            println!("  {}{}{}{}", m, u, x, a);
            println!("×       {}", a);
            println!("--------");
            println!("  {}{}{}{}", s, l, o, n);
            println!();
            count += 1;
        }
    }

    println!("Total solutions: {}", count);
}
