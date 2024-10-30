use std::collections::HashSet;

fn main() {
    let mut count = 0;

    for m in 1..=8 {
        for u in 1..=8 {
            for x in 1..=8 {
                for a in 1..=8 {
                    for s in 1..=8 {
                        for l in 1..=8 {
                            for o in 1..=8 {
                                for n in 1..=8 {
                                    let letters = vec![m, u, x, a, s, l, o, n];
                                    let unique: HashSet<_> = letters.iter().cloned().collect();
                                    if unique.len() == 8 {
                                        println!("  {}{}", m, u);
                                        println!("{}    {}", x, a);
                                        println!("------");
                                        println!("    {}{}{}{}", s, l, o, n); 
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Total solutions: {}", count);
}
