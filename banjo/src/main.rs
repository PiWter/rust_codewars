fn main() {
    let name = "Alfredo";
    let result = are_you_playing_banjo1(name);
    println!("{:?}", result);
}

fn are_you_playing_banjo1(name: &str) -> String {
    match name.to_lowercase().starts_with("r") {
        true => format!("{} plays banjo", name),
        false => format!("{} does not play banjo", name)
    }
}

fn are_you_playing_banjo(name: &str) -> String {
    if name.chars().next() == Some('R') || name.chars().next() == Some('r') {       //no es tan óptimo como el de arriba
        name.to_owned() + &" plays banjo".to_string()
    } else {
        name.to_owned() + &" does not play banjo".to_string()
    }
}