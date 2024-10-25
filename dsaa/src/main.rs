
fn swap_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap_or(c)
            } else {
                c.to_uppercase().next().unwrap_or(c) 
            }
        })
        .collect()
}

fn main() {
    let text = "Hello, world!"; 
    let swapped = swap_case(text);
    println!("{}", swapped); 
}
