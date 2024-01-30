use std::io;

fn main() {
    println!("Introduce a number");
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line");

    let n: i32 = n.trim().parse().expect("Your input wasn't a number");

    println!("{}", n * 2);
}