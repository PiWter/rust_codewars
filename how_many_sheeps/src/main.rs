fn main() {
    let sheep = [true,  true,  true,  false,
    true,  true,  true,  true ,
    true,  false, true,  false,
    true,  false, false, true ,
    true,  true,  true,  true ,
    false, false, true,  true];

    count_sheep(&sheep);
    let result = count_sheep(&sheep);
    println!("{:?}", result);


}


fn count_sheep(sheep: &[bool]) -> u8 {
    let mut counter: u8 = 0;
      
      for x in sheep {
        if x == &true {
            counter += 1;
        }
      }
      counter
  }