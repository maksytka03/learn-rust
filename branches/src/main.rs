mod fibonacci;

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is {number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The values is {index}");

        index += 1;
    }

    for element in a {
        println!("The values is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}
