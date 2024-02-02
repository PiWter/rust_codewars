fn main() {
    let s1: u16 = 90;
    let s2: u16 = 90;
    let s3: u16 = 90;

    println!("{}", get_grade(s1, s2, s3))
}

fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    let result = (s1 + s2 + s3) / 3;
    
    match result {
        (90..=100) => 'A',
        (80..=90) => 'B',
        (70..=80) => 'C',
        (60..=70) => 'D',
        (0..=60) => 'F',
        _ => 'F'
    }
}
