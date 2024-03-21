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

    // Extract the line number from the first line (assuming valid format)
    let line_number: usize = lines[0].parse().expect("Error parsing line number");

    // Create an empty vector for space values
    let mut list_of_spaces: Vec<i32> = Vec::new();

    // Extract space values from lines 1 to line_number (inclusive)
    for spaces in lines.iter().skip(1).take(line_number) {
        list_of_spaces.push(spaces.parse().expect("this is not a string"))
    }

    let mut number_of_presses: i128 = 0;
    for spaces in list_of_spaces {
        number_of_presses += get_presses_for_spaces(spaces);
    }

    println!("{}", number_of_presses);
}
fn get_presses_for_spaces(spaces: i32) -> i128 {
    let number_of_tabs = spaces / 4;
    let remainder = spaces % 4;
    let remainder_presses: i32 = match remainder {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 2,
        _ => 0,
    };
    (number_of_tabs + remainder_presses) as i128
}
