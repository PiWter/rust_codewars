fn main() {
    let input_array = [2, 3, 1];
    println!("{}", gimme(input_array));
}



fn gimme(input_array: [i32;3]) -> usize {
    
    fn array_sort(mut input_array: [i32;3]) -> i32 {
        input_array.sort();
        input_array[1]
    }
    
    let n = array_sort(input_array);

    input_array.iter().position(|&r| r == n).unwrap()

}

/*

fn gimme(input_array: [i32;3]) -> usize {
    let mut ret = input_array.clone();
    ret.sort();
    input_array.iter().position(|&x| x == ret[1]).unwrap()
}

*/