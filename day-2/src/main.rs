use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let max_values = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut total = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            total += validate_line(line, &max_values);
        }
    }

    println!("total {}", total);
}

fn validate_line(line: String, max_values: &HashMap<&str, i32>) -> u32 {
    let (id, offset) = game_id(&line);
    let rest = &line[offset + 2..];

    for set in rest.split("; ") {
        for combination in set.split(", ") {
            let mut amount_string = String::new();
            let mut amount = 0;
            let mut index = 0;

            for char in combination.chars() {
                if char == ' ' {
                    amount = amount_string.parse().unwrap();
                    break;
                }

                amount_string.push(char);
                index += 1;
            }

            let rest = &combination[index + 1..];
            let max = max_values[rest];

            if amount > max {
                return 0;
            }
        }
    }

    return id;
}

fn game_id(line: &String) -> (u32, usize) {
    let line_slice = &line[5..];
    let mut id_string = String::new();
    let mut index = 5;

    for char in line_slice.chars() {
        if char == ':' {
            break;
        }

        id_string.push(char);
        index += 1;
    }

    let parsed: u32 = id_string.parse().unwrap();

    (parsed, index)
}
