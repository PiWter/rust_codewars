fn main() {
    let string: &str = "a, e, i, o, u";
    //let string = "let's all love lain";
    //let result = get_count(string);
    //println!("{}", result);
    get_count(string);
}

fn get_count(string: &str) {
    let string = string.matches(|x| match x {'a'|'e'|'i'|'o'|'u' => true, _ => false}).count();
    //let string = string.matches(|x| match x {'l' => true, _ => false}).count();
    //let string = string.matches(|x| match x {'a' => true, _ => false});

    println!("{:?}", string);
}