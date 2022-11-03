// https://www.codewars.com/kata/5b73fe9fb3d9776fbf00009e/rust

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.is_empty() || arr.len() == 1 { return None; }
    return Some(arr.iter().max().unwrap() - arr.iter().min().unwrap());
}

fn main() {
    println!("{:?}", sum_of_differences(&[]));
}
