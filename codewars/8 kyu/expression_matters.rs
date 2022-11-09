// https://www.codewars.com/kata/5ae62fcf252e66d44d00008e/rust

fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    *[a * b * c, a + b + c, (a + b) * c, a * (b + c)].iter().max().unwrap()
}

fn main() {
    println!("{}", expressions_matter(5, 3, 1));
}
