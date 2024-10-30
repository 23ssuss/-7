use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32, usize, usize, i32)> {
    if data.len() < 2 {
        return None; 
    }

    let mut min_sum = i32::MAX;
    let mut min_pair = (0, 0);
    let mut min_indexes = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (data[i], data[i + 1]);
            min_indexes = (i, i + 1);
        }
    }

    Some((min_pair.0, min_pair.1, min_indexes.0, min_indexes.1, min_sum))
}

fn print_vector_info(data: &[i32]) {
    let len = data.len();
    
    print!("indexes: ");
    for i in 0..len {
        print!("{} ", i);
    }
    println!();

    println!("data: {:?}", data);

    if let Some((a, b, idx1, idx2, sum)) = min_adjacent_sum(data) {
        println!("min adjacent sum={}+{}={} at indexes:{},{}", a, b, sum, idx1, idx2);
        
        print!("indexes: ");
        for i in 0..len {
            if i == idx1 || i == idx2 {
                print!("\\__ ");
            } else {
                print!("   ");
            }
        }
        println!();
    } else {
        println!("Not enough elements to find a pair.");
    }
}

fn main() {
    for _ in 0..4 {
        let random_vector = gen_random_vector(20);
        print_vector_info(&random_vector);
        println!();
    }
}
