use std::io;

fn main() {
    let mut choice = String::new();
    let mut degrees = String::new();
    let input_text = "Enter degrees";

    println!("Enter (C) to transform degrees Fahrenheit to Celsius and (F) to do the inverse:");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    println!("{input_text}");

    io::stdin()
        .read_line(&mut degrees)
        .expect("Failed to read line");

    choice = choice.trim().to_string();

    let mut degrees: f32 = degrees.trim().parse().expect("Please enter a number.");

    degrees = if choice.to_uppercase() == "C" {
        (degrees - 32.0) * 0.5556
    } else {
        degrees * 0.5556 + 32.0
    };
    println!("Result: {degrees}");
}
