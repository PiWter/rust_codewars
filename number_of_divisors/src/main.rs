fn main() {
    let n = 30;
    let result = divisors(n);
    println!("{}", result);
}

/*fn divisors(n: u32) -> u32 {          //Este es el que he hecho
    let mut counter = 1;
    let mut div = 0;
    loop {

        if n % counter == 0 {
            div += 1
        }

        if n == counter {
            break
        }
        counter += 1;
    };

    div
}*/

fn divisors(n: u32) -> u32 {
    (1..=n).filter(|x| n % x == 0).count() as u32               //Este es el más óptimo
}

/*fn divisors(n: u32) -> u32 {              //Este es el que podría haber hecho
    let mut res = 0;

    for d in 1..=n {
        if n % d == 0 {
            res += 1;
        };
    }

    res
}*/
