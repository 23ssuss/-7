use std::io;

fn is_palindrome(number: u32) -> bool {
    let original = number.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

fn main() {
    println!("Введите число для проверки, является ли оно палиндромом:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let number: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Пожалуйста, введите корректное число.");
            return;
        }
    };

    if is_palindrome(number) {
        println!("{} является палиндромом.", number);
    } else {
        println!("{} не является палиндромом.", number);
    }
}
