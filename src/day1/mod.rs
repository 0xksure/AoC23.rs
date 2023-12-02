use std::{error, fs::File};

use csv::{Error, ReaderBuilder};

pub fn get_first_and_last_digit_of_string(code: &str) -> u64 {
    let mut digits = code.chars().filter(|c| c.is_numeric());
    let first_digit = digits.next().unwrap();
    let last_digit = match digits.last() {
        Some(digit) => digit,
        None => first_digit,
    };
    let combined_char_digit = format!("{}{}", first_digit, last_digit);
    return combined_char_digit.parse::<u64>().unwrap();
}

pub fn get_sum_of_digits() -> Result<u64, Error> {
    let input_document = std::fs::read_to_string("src/day1/code.csv").unwrap();
    let mut sum = 0;
    for line in input_document.lines() {
        let first_and_last_digit = get_first_and_last_digit_of_string(line);
        println!("First and last digit: {} of {}", first_and_last_digit, line);
        sum += first_and_last_digit;
    }
    return Ok(sum);
}
