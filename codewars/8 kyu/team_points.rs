// https://www.codewars.com/kata/5bb904724c47249b10000131/rust

fn main() {
    let array = &["1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3"]
        .iter().map(|x| x.to_string()).collect::<Vec<_>>();
    println!("{}", points(array));
}

fn points(games: &[String]) -> u32 {
    let mut scores = 0;
    for i in 0..games.len() {
        let points: Vec<&str> = games[i].split(":").collect();
        let x = points[0];
        let y = points[1];
        if x > y { scores += 3 } else if x == y { scores += 1 }
    }
    return scores;
}
