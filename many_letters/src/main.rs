use std::io;

fn main() {
    let mut n = String::new();
    let letter: &str = "I";

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line");

    let n: u32 = n.trim().parse().expect("Your input is not a valid number");

    println!("{}", letter.repeat(n.try_into().unwrap()));

}