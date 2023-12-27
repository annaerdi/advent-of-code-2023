use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::HashMap;

pub fn part_2(filename: &str) {
    let file = File::open(filename);
    let mut contents = String::new();
    let mut sum = 0;
    let _ = file.expect("error reason").read_to_string(&mut contents);

    let re = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|ten|[0-9])").unwrap();

    let mut word_to_digit = HashMap::new();
    word_to_digit.insert("one", '1');
    word_to_digit.insert("two", '2');
    word_to_digit.insert("three", '3');
    word_to_digit.insert("four", '4');
    word_to_digit.insert("five", '5');
    word_to_digit.insert("six", '6');
    word_to_digit.insert("seven", '7');
    word_to_digit.insert("eight", '8');
    word_to_digit.insert("nine", '9');

    // iterate over each line in the contents -> lines() is splitting the contents by newline characters
    for line in contents.lines() {

        let mut number_string = String::new();
        let mut start = 0;

        while let Some(mat) = re.find(&line[start..]) {
            let matched = &line[start..][mat.start()..mat.end()];
            let digit = match word_to_digit.get(matched) {
                Some(&d) => d,
                None => matched.chars().next().unwrap(),
            };
            number_string.push(digit);
            start += mat.start() + 1;
        }

        let first_digit = number_string.chars().next().unwrap();
        let last_digit = number_string.chars().last().unwrap();
        let two_digit_string = format!("{}{}", first_digit, last_digit);
        let two_digit_number: i32 = two_digit_string.parse().unwrap();
        sum += two_digit_number;
        //println!("line: {}, number extraced: {} ---- two digit ---> {}", line, number_string, two_digit_number);

    }
    println!("{}", sum);
}
