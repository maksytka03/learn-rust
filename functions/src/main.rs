fn main() {
    println!("Hello, world!");

    print_label_measurement(5, 'h');

    let x = five();
    println!("{x}");
    println!("Plus one: {}", plus_one(x))
}

fn print_label_measurement(x: i32, unit_label: char) {
    println!("The value of x is {x}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}