fn main() {
    let vec = vec![1, 2, 4];
    //let result = square_sum(vec);
    //println!("{:?}", result);
    square_sum(vec);
}

fn square_sum(vec: Vec<i32>) {
    let result: i32 = vec.iter().map(|s| s * s).sum();
    println!("{:?}", result);
}

fn square_sum1(vec: Vec<i32>) -> i32 {
    vec.iter().map(|s| s * s).sum()
}