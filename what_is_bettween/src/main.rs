use std::vec::Vec;

fn main() {
    let a = 1;
    let b = 20;

    //let range: Vec<i16> = (a..=b).collect();
    let range = (a..=b).rev().collect::<Vec<u32>>();
    println!("{:?}", range);
}