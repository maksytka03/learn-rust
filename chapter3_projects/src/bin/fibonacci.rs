use std::io;

fn generate_fibonacci(n: usize) -> Vec<u64> {
    let mut fibonacci_sequence = Vec::new();

    if n >= 1 {
        fibonacci_sequence.push(0);
    }
    if n >= 2 {
        fibonacci_sequence.push(1);
    }

    for i in 2..n {
        let next_fibonacci = fibonacci_sequence[i - 1] + fibonacci_sequence[i - 2];
        fibonacci_sequence.push(next_fibonacci);
    }

    fibonacci_sequence
}

fn main() {
    loop {
        println!("Enter which number of the fibonacci sequence to get (enter 'q' to quit):");
        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("Failed to read");
        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        let fib = generate_fibonacci(index)[index-1];
        println!("The number is: {}", fib);
    }
}
