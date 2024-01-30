fn main() {
    let s: &str = "{[))]}";
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            ')' => if stack.pop() != Some('(') { println!("false"); },
            '{' => stack.push(c),
            '}' => if stack.pop() != Some('{') { println!("false"); },
            '[' => stack.push(c),
            ']' => if stack.pop() != Some('[') { println!("false"); },
            _ => panic!("Invalid input")
        }
    }
}
