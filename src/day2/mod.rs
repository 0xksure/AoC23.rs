use std::{collections::HashMap, fmt::format, io::Error, io::ErrorKind, vec};

pub fn get_sum_of_digits() -> Result<u64, Error> {
    let input_document = std::fs::read_to_string("src/day1/code.csv").unwrap();
    let mut sum = 0;
    for line in input_document.lines() {
        let first_and_last_digit = get_first_and_last_digit_letters(line)?;
        println!("First and last digit: {} of {}", first_and_last_digit, line);
        sum += first_and_last_digit;
    }
    return Ok(sum);
}

fn matching_in_list(code: &str, valid_number_strings: Vec<&&str>) -> (bool, bool) {
    for number in valid_number_strings {
        if number.starts_with(code) {
            return (true, number.eq(&code));
        }
    }
    return (false, false);
}

fn get_first_and_last_digit_letters(code: &str) -> Result<u64, Error> {
    let mut valid_number_strings = HashMap::new();
    valid_number_strings.insert("one", 1);
    valid_number_strings.insert("two", 2);
    valid_number_strings.insert("three", 3);
    valid_number_strings.insert("four", 4);
    valid_number_strings.insert("five", 5);
    valid_number_strings.insert("six", 6);
    valid_number_strings.insert("seven", 7);
    valid_number_strings.insert("eight", 8);
    valid_number_strings.insert("nine", 9);
    valid_number_strings.insert("1", 1);
    valid_number_strings.insert("2", 2);
    valid_number_strings.insert("3", 3);
    valid_number_strings.insert("4", 4);
    valid_number_strings.insert("5", 5);
    valid_number_strings.insert("6", 6);
    valid_number_strings.insert("7", 7);
    valid_number_strings.insert("8", 8);
    valid_number_strings.insert("9", 9);
    let len_code = code.len();
    let mut number_matches = vec![];
    let mut i = 0;
    let mut j = 1;
    while i < len_code {
        while j <= len_code {
            let subset = &code[i..j];
            let is_match =
                matching_in_list(subset, valid_number_strings.keys().collect::<Vec<&&str>>());
            if is_match.1 {
                number_matches.push(subset);
                i = j;
                break;
            }
            if !is_match.0 {
                i += 1;
                break;
            }
            j += 1;
        }
        if j > len_code {
            i += 1;
        }
    }
    if number_matches.len() == 0 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("No match found for number in {}", code),
        ));
    }

    let first_match = match valid_number_strings.get(number_matches.first().unwrap()) {
        Some(number) => number,
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "No match found for first number",
            ))
        }
    };

    let last_match = match valid_number_strings.get(number_matches.last().unwrap()) {
        Some(number) => number,
        None => {
            return Err(Error::new(
                ErrorKind::Other,
                "No match found for last number",
            ))
        }
    };

    return Ok(format!("{}{}", first_match, last_match)
        .parse::<u64>()
        .unwrap());
}

// test code
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_and_last_digit_of_string() {
        assert_eq!(get_first_and_last_digit_letters("1234").unwrap(), 14);
        assert_eq!(get_first_and_last_digit_letters("12345").unwrap(), 15);
        assert_eq!(get_first_and_last_digit_letters("two1nine").unwrap(), 29);
        assert_eq!(
            get_first_and_last_digit_letters("eightwothree").unwrap(),
            83
        );
        assert_eq!(
            get_first_and_last_digit_letters("abcone2threexyz").unwrap(),
            13
        );
        assert_eq!(get_first_and_last_digit_letters("xtwone3four").unwrap(), 24);
        assert_eq!(
            get_first_and_last_digit_letters("4nineeightseven2").unwrap(),
            42
        );
        assert_eq!(get_first_and_last_digit_letters("zoneight234").unwrap(), 14);
        assert_eq!(
            get_first_and_last_digit_letters("7pqrstsixteen").unwrap(),
            76
        );
    }

    #[test]
    fn test_get_first_and_last_digit_letters() {
        assert_eq!(get_first_and_last_digit_letters("one").unwrap(), 11);
    }
}