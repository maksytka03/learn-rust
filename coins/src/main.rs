enum UsState {
    Alabama,
    Alaska,
    Arkansas,
    Illinois,
    Washington
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {println!("State quarter from {:?}!", state; 25)},
    }
}

fn main() {
    println!("Hello, world!");
}
