use std::time::Instant;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let mut l = vec![];
    {
        for i in 0..10000000 {
            for j in 0..20 {
                l.push(j)
            }
        }
    }
    let e = now.elapsed();
    println!("{:?}", e);

    println!("-------------\n");

    let s = String::from("NOGOISGO:IDSGDFIOGHDG");

    for i in 0..2 {
        println!("{}", s.chars().nth(i).unwrap())
    }
}
