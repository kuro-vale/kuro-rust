// https://www.codewars.com/kata/5d59576768ba810001f1f8d6/rust

pub fn quadratic(x1: i32, x2: i32) -> (i32, i32, i32) {
    let a = 1;
    let b = -x1 + -x2;
    let c = x1 * x2;
    return (a, b, c);
}
