use std::{
    char,
    fs::File,
    io::{BufReader, BufRead},
};

const NUMBER_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut total = 0;

    for line in lines {
        if let Ok(line) = line {
            total += get_number(&line);
        }
    }

    println!("Total {}", total);
}

fn get_number(line: &String) -> u32 {
    let mut start = '0';
    let mut end = '0';
    let mut word = String::new();

    for char in line.chars() {
        word.push(char);

        if let Some(number) = parse_number_string(&word) {
            if start == '0' {
                start = number;
            }

            end = number;
            continue;
        }

        if let Some(_) = char.to_digit(10) {
            if start == '0' {
                start = char;
            }

            end = char;
        }
    }

    let result = format!("{}{}", start, end);
    result
        .parse()
        .expect(format!("{} is not a number", result).as_str())
}

fn parse_number_string(string: &String) -> Option<char> {
    for (index, number_string) in NUMBER_STRINGS.iter().enumerate() {
        if string.ends_with(number_string) {
            return char::from_digit((index + 1) as u32, 10);
        }
    }

    return None;
}
