use std::fs::File;
use std::io::BufRead;

fn main() {
    // Define the file path
    let file_path = "input.txt";

    // Open the file in read mode and handle potential errors
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("Error opening file: {}", err),
    };

    // Create a buffered reader
    let reader = std::io::BufReader::new(file);

    // Collect lines from the reader into a vector of strings
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect();
    let mut number_of_numbers = 0;
    let mut numbers: Vec<i32> = Vec::new();
    for (position, line) in lines.iter().take(2).enumerate() {
        match position {
            0 => number_of_numbers = line.parse().unwrap(),
            1 => {
                numbers = line
                    .split_ascii_whitespace()
                    .take(number_of_numbers)
                    .map(|s| s.parse().unwrap())
                    .collect()
            }
            _ => (),
        }
    }

    let mut previous_number_even: bool = false;
    let mut current_number_even: bool;
    let mut symbols: Vec<char> = Vec::new();
    for (position, number) in numbers.iter().enumerate() {
        if position == 0 {
            previous_number_even = (number % 2) == 0;
        } else {
            current_number_even = (number % 2) == 0;
            let (symbol, sum_is_even) = match (previous_number_even, current_number_even) {
                (true, true) => ('+', true),
                (true, false) => ('+', false),
                (false, true) => ('+', false),
                (false, false) => ('x', false),
            };
            symbols.push(symbol);
            previous_number_even = sum_is_even;
        }
    }
    println!("{}", symbols.iter().collect::<String>());
}
