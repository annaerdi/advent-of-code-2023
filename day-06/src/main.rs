/// Input:
/// ------------------------------------
/// Time:        53     89     76     98
/// Distance:   313   1090   1214   1201


fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let a = calculate_number_of_ways(53, 313);
    let b = calculate_number_of_ways(89, 1090);
    let c = calculate_number_of_ways(76, 1214);
    let d = calculate_number_of_ways(98, 1201);
    println!("Result for part 1 is {}", a*b*c*d);
}

fn part_2() {
    println!("Result for part 2 is {}", calculate_number_of_ways(53897698, 313109012141201));
}

fn calculate_number_of_ways(time: i64, best_distance: i64) -> i64 {
    let mut number_of_ways = 0;

    for holding_time in 1..time {
        let seconds_left = time - holding_time;
        let distance = holding_time * seconds_left; // holding_time is also the speed
        if distance > best_distance {
            number_of_ways += 1;
        }
    }

    number_of_ways

}



