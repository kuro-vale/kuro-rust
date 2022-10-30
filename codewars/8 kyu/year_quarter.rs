// https://www.codewars.com/kata/5ce9c1000bab0b001134f5af/rust

fn quarter_of(month: u8) -> u8 {
    return (month as f32 / 3.0).ceil() as u8;
}
