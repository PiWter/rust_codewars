fn main() {
    let text: String = "   hello world ".to_string();

    println!("The original value is {text}");
    println!("The value without spaces at the start and at the end is {}", text.trim());
    println!("The value without any spaces is {}", text.replace(" ", ""));
}
