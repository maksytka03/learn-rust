use std::fs;

fn main() {
    let contents = fs::read_to_string("1.txt").expect("Failed");
    let mut sum = 0;
    for line in contents.lines() {
        for i in line.chars().collect() {
            if i.is_digit() {
                sum += i
            }
        }
    }
    println!("{}", sum);
}
