fn draw_level(width: usize, max_width: usize) {
    let stars = "*".repeat(width);
    let spaces = " ".repeat((max_width - width) / 2);
    println!("{spaces}{stars}{spaces}");
}

fn draw_tree(num_triangles: usize) {
    let max_width = 2 * (num_triangles + 1) - 1;


    draw_level(1, max_width);


    for i in 1..=num_triangles {
        for j in 0..=i {
            draw_level(2 * j + 1, max_width);
        }
    }


    for _ in 0..3 {
        draw_level(1, max_width);
    }
}

fn main() {
    let num_triangles = 5;
    draw_tree(num_triangles);
}