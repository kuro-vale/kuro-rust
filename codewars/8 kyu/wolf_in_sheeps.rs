// https://www.codewars.com/kata/5c8bfa44b9d1192e1ebd3d15/rust

fn warn_the_sheep(queue: &[&str]) -> String {
    let mut array: Vec<&str> = queue.to_vec();
    array.reverse();
    if array[0] == "wolf" {
        return "Pls go away and stop eating my sheep".to_string();
    }
    for i in 0..array.len() {
        if array[i] == "wolf" {
            return format!("Oi! Sheep number {i}! You are about to be eaten by a wolf!");
        }
    }
    return "There is no wolf".to_string();
}
