use std::fs::File;
use std::io::Read;

pub fn part_1(filename: &str) {
    let file = File::open(filename);
    let mut contents = String::new();
    let mut sum = 0;

    // read the entire contents of the file into a string
    let _ = file.expect("error reason").read_to_string(&mut contents);
    // note: this `Result` may be an `Err` variant, which should be handled
    //     = note: `#[warn(unused_must_use)]` on by default
    // help: use `let _ = ...` to ignore the resulting value

    // iterate over each line in the contents -> lines() is splitting the contents by newline characters
    for line in contents.lines() {
        let mut digits = String::new();
        // collect all digits into a string
        for c in line.chars() {
            if c.is_ascii_digit() {
                digits.push(c);
            }
        }
        let first_digit = digits.chars().next().unwrap();
        let last_digit = digits.chars().last().unwrap();
        let two_digit_string = format!("{}{}", first_digit, last_digit);
        let two_digit_number: i32 = two_digit_string.parse().unwrap();
        sum += two_digit_number;
    }
    println!("{}", sum);
}
