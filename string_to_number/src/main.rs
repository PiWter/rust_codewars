fn main() {
    let age = "2 years old";
    println!("{}", get_age(age));
}
 
fn get_age(age: &str) -> u32 {
    age.chars().next().unwrap() as u32 - 48
}

/*fn get_age(age: &str) -> u32 {
    age[..1].parse().unwrap()
} */

/*fn get_age(age: &str) -> u32 {
    age[..1].parse::<u32>().unwrap() 
} */