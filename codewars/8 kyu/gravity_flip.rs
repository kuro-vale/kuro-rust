// https://www.codewars.com/kata/5f70c883e10f9e0001c89673/rust

fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut array: Vec<u32> = cubes.to_vec();
    return if dir == 'L' {
        array.sort();
        array.reverse();
        array
    } else if dir == 'R' {
        array.sort();
        array
    } else {
        println!("Please enter a valid direction ('L' or 'R')");
        array
    };
}
  
