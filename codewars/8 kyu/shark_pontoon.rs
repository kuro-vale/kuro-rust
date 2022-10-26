fn shark(pontoon_distance: f64, shark_distance: f64,
         you_speed: f64, mut shark_speed: f64, dolphin: bool) -> String {
    if dolphin {
        shark_speed /= 2.0;
    }
    let shark_eat_time = shark_distance / shark_speed;
    let safe_time = pontoon_distance / you_speed;

    return if shark_eat_time < safe_time {
        "Shark Bait!"
    } else {
        "Alive!"
    }.to_string();
}
