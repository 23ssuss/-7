fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    let n = n.rem_euclid(len) as usize;
    
    let (part1, part2) = s.split_at(len as usize - n);
    format!("{}{}", part2, part1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.to_string(), *n), exp.to_string());
        });
    }
}

fn main() {
    let s = "abcdefgh".to_string();
    let n = 2; 
    let rotated = rotate(s, n);
    println!("Rotated string: {}", rotated);
}
