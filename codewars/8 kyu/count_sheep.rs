// https://www.codewars.com/kata/5b077ebdaf15be5c7f000077/rust

fn count_sheep(n: u32) -> String {
    let mut sheep = "".to_string();
    for i in 1..=n {
        sheep += &*(i.to_string() + " sheep...");
    }
    return sheep;
}

fn main() {
    println!("{}", count_sheep(2));
}
