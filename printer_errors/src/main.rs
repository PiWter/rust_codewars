fn main() {
    let s = "aaaaxxxpbbmt";
    println!("{}", printer_error(s));
}

fn printer_error(s: &str) -> String {
    let mut errors = 0;

    for s in s.chars() {
        if !('a'..='m').contains(&s) { errors += 1 }
    }

    format!("{}/{}", errors.to_string(), s.chars().count())
}

// format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())        <- Más optimizado