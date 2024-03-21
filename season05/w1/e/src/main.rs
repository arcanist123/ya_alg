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

    let mut line_components: Vec<&str> = Vec::new();
    for line in lines.iter().take(1) {
        line_components = line.split(' ').into_iter().take(3).collect();
    }

    let number_to_be_divided = line_components[0];
    let divisor: i32 = line_components[1].parse().unwrap();
    let number_of_days: usize = line_components[2].parse().unwrap();
    let mut is_number_found = false;
    for i in 0..=9 {
        let number_to_be_divided_next = format!("{}{}", number_to_be_divided, i);
        let divident: i32 = number_to_be_divided_next.parse().unwrap();
        if (divident % divisor) == 0 {
            println!(
                "{}{}",
                number_to_be_divided_next,
                "0".repeat(number_of_days - 1)
            );
            is_number_found = true;
            break;
        }
    }
    if !is_number_found {
        println!("-1");
    }
}
