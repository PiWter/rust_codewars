fn main() {
    let word = "Let's all love Lain";
    let ending = "Lain";

    let result = solution(word, ending);
    println!("{}", result);
}

fn solution(word: &str, ending: &str) -> bool {
    let characters = ending.len();
    //let characters = 0;

    //for _c in ending.chars() {
    //    characters += 1;
    //}

    let word_end = &word[word.len() - characters..];

    word_end == ending
}