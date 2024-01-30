fn main() {
    println!("Hello, world!");
    let n = 25;
    let result = is_square1(n);
    println!("{}", result);
}

fn is_square1(n: i64) -> bool {
    println!("{}", (n as f64).sqrt().fract());
    (n as f64).sqrt().fract() == 0.0
}

fn is_square2(n: i64) -> bool {
    ((n as f64).sqrt() as i64).pow(2) == n
}

fn is_square(n: i64) -> bool {
    let limite = (n as f64).sqrt() as i64;      //De nuevo no es la más óptima
    println!("{}", limite);      
    if limite * limite == n {
        true
    } else {
        false
    }
}