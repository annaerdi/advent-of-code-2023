mod part_1;
mod part_2;

const FILENAME: &str = "input.txt";

fn main() {
    println!("Result of part 1:");
    part_1::part_1(FILENAME);
    println!("Result of part 2:");
    part_2::part_2(FILENAME);
}
