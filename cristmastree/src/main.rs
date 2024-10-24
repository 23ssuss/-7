fn main() {
    let triangles: usize = 5; 
    draw_tree(triangles);
}

fn draw_tree(triangles: usize) {
    let mut width = 1;
    let max_height = triangles + 2; 
    let max_width = width + 2 * (triangles + 2); 

    (0..triangles).for_each(|triangle| {
        let height = triangle + 3; 
        (0..height).for_each(|i| {
            let stars = width + 2 * i;
            let spaces = (max_width.saturating_sub(stars)) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        });
        width += 2; 
    });
}
