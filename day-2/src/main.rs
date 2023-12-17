use std::{fs::File, io::BufRead, io::BufReader};

#[derive(Debug)]
struct RGB {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl RGB {
    pub fn empty() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn from(r: u32, g: u32, b: u32) -> Self {
        Self { r, g, b }
    }

    pub fn set(&mut self, key: &char, value: u32) {
        match key {
            'r' => self.r = value,
            'g' => self.g = value,
            'b' => self.b = value,
            _ => (),
        }
    }

    pub fn get(&self, key: &char) -> u32 {
        match key {
            'r' => self.r,
            'g' => self.g,
            'b' => self.b,
            _ => 0,
        }
    }

    pub fn lt(&self, other: &RGB) -> bool {
        if self.r > other.r {
            return false;
        }

        if self.g > other.g {
            return false;
        }

        if self.b > other.b {
            return false;
        }

        return true;
    }

    pub fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let max_values = RGB::from(12, 13, 14);
    let mut total = 0;
    let mut power = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let (id, values) = parse_game(line);

            power += values.power();

            if values.lt(&max_values) {
                total += id;
            }
        }
    }

    println!("total {}", total);
    println!("power {}", power);
}

fn parse_game(line: String) -> (u32, RGB) {
    let mut values = RGB::empty();
    let (id, offset) = game_id(&line);
    let line = &line[offset + 2..];

    for set in line.split("; ") {
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

            let color = &combination[index + 1..].chars().nth(0).unwrap();
            values.set(color, std::cmp::max(values.get(color), amount));
        }
    }

    (id, values)
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
