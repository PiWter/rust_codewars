use std::io;

fn main() {
    let mut n = String::new();

    println!("Type a number");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line");

    let n: i32 = n.trim().parse().expect("This is not a number");

    println!("{}", n * -1);
}
