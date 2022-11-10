// https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097/rust

fn century(year: u32) -> u32 {
    return (year as f32 / 100.0).ceil() as u32;
}

fn main() {
    println!("{}", century(2001));
}
