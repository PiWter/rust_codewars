fn main() {
    let arr_a = ["a", "a", "c", "b"];
    let arr_b = ["a", "a", "b", ""];
    println!("{:?}", check_exam(&arr_a, &arr_b))
}

fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut counter = 0;
    let mut score = 0;

    for _c in arr_a {
        if &arr_a[counter] == &arr_b[counter] {score += 4;} else if &arr_b[counter] == &"" {score += 0} else {score -= 1;}
        counter +=1
    }

    if score < 0 {return 0}
    score
}

/*          BETTER SOLUTION
fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    arr_a.iter().zip(arr_b.iter()).fold(0, |pts, ans| {
        match ans {
            (a, b) if a == b => pts + 4, 
            (_, b) if b == &"" => pts, 
            _ => pts - 1
        }
    }).max(0)
}
*/