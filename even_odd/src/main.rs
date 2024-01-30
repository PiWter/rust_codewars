use std::io;
use rand::Rng;

fn main() {

    println!("Input a number: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Is not a number");

    if n % 2 == 0 {
        println!("Odd");
        println!("{}", n * 8);
    } else {
        println!("Even");
        println!("{}", n * 9);
    }
}
