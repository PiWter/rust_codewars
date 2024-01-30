fn main() {
    let beast = "great blue heron";
    let dish = "garlic naan";

    let result = feast(beast, dish);
    println!("{}", result);
}

fn feast(beast: &str, dish: &str) -> bool {
    beast.chars().next() == dish.chars().next() 
    && beast.chars().last() == dish.chars().last()
}

fn feast1(beast: &str, dish: &str) -> bool {        //Este funciona pero el de arriba es más óptimo
    let mut f: char = 'a';
    let mut l: char = 'a';
    let mut fd = 'a';
    let mut ld = 'a';
    for c in beast.chars() {
        f = c;
        break
    }

    for c in beast.chars() {
        l = c;
    }

    for c in dish.chars() {
        fd = c;
        break
    }

    for c in dish.chars() {
        ld = c;
    }
    
    println!("{}", f);
    println!("{}", l);
    println!("{}", fd);
    println!("{}", ld);

    if f == fd && l == ld {
        return true
    } else {
        return false
    }
}
