use rand::Rng;
use std::convert::TryInto;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn count_permutation(shipments: &Vec<u32>) -> Result<usize, String> {
    let total: u32 = shipments.iter().sum();
    if total % shipments.len() as u32 != 0 {
        return Err("Невозможно равномерно распределить груз".to_string());
    }
    let average = total / shipments.len() as u32;
    let mut transfers = 0;
    for &weight in shipments {
        if weight > average {
            transfers += weight - average;
        }
    }
    transfers.try_into().map_err(|_| "Ошибка преобразования".to_string())
}

fn display_shipments(shipments: &Vec<u32>, transfers: usize) {
    let total: u32 = shipments.iter().sum();
    let average = total / shipments.len() as u32;
    println!("Индексы: {:?}", (0..shipments.len()).collect::<Vec<_>>());
    println!("Данные: {:?}", shipments);
    println!("Общий вес: {}", total);
    println!("Средний вес: {}", average);
    println!("Минимальное количество переносов: {}", transfers);
}

fn main() {
    let n = 20;
    let shipments = gen_shipments(n);
    match count_permutation(&shipments) {
        Ok(transfers) => {
            display_shipments(&shipments, transfers);
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

# Програма для розрахунку мінімальної передачі вантажу на кораблях

## Опис
Ця програма генерує випадкові ваги вантажів на кораблях і розраховує, скільки мінімально потрібно перенести вантажу, щоб на всіх кораблях була однакова вага.

## Приклади

1. **Вхідні дані:** 
   - Вантажі: `[8, 2, 2, 4, 4]`
   - Загальна вага: `20`
   - Середня вага: `4`
   
   **Процес:**
   - Потрібно перемістити:
     - -2 (з 8 в 4)
     - +2 (з 2 в 4)
   
   **Відповідь:** `4`

2. **Вхідні дані:**
   - Вантажі: `[9, 3, 7, 2, 9]`
   - Загальна вага: `30`
   - Середня вага: `6`
   
   **Процес:**
   - Потрібно перемістити:
     - -3 (з 9 в 6)
     - +3 (з 3 в 6)
     - -1 (з 7 в 6)
     - +1 (з 2 в 6)
   
   **Відповідь:** `7`

## Запуск програми
1. Переконайтесь, що у вас встановлені Rust та Cargo.
2. Клонуйте цей репозиторій.
3. Запустіть команду `cargo run` для виконання програми.
