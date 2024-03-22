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
    let input: Vec<i32> = reader
        .lines()
        .map(|line| line.expect("Error reading line").parse().unwrap())
        .collect();

    let my_soldjers = input[0];
    let building_health = input[1];
    let enemy_soldjers_per_round = input[2];

    get_number_of_rounds(
        &State {
            my_soldjers: 0,
            building_health: 0,
            enemy_soldjers: 0,
            enemy_soldjers_per_round: 0,
        },
        &State {
            my_soldjers,
            building_health,
            enemy_soldjers: 0,
            enemy_soldjers_per_round,
        },
        0,
    );
}

struct State {
    my_soldjers: i32,
    building_health: i32,
    enemy_soldjers: i32,
    enemy_soldjers_per_round: i32,
}

fn get_number_of_rounds(old_state: &State, state: &State, current_round: i32) -> i32 {
    i32::min(
        get_after_building_attack(old_state, state, current_round),
        get_after_soldjer_attack(old_state, state, current_round),
    )
}

fn get_after_building_attack(old_state: &State, state: &State, current_round: i32) -> i32 {
    //my move
    //first attack building
    let mut building_health = state.building_health - state.my_soldjers;
    if building_health < 0 {
        building_health = 0
    }
    let my_remaining_soldjers = state.my_soldjers - state.building_health;

    //then attack soldjers
    let mut remaining_enemy_soldjers: i32 = state.enemy_soldjers - my_remaining_soldjers;
    if remaining_enemy_soldjers < 0 {
        remaining_enemy_soldjers = 0
    }
    //enemy move
    let my_surviving_soldjers = state.my_soldjers - remaining_enemy_soldjers;
    let mut enemy_soldjers = remaining_enemy_soldjers;
    if building_health > 0 {
        enemy_soldjers += state.enemy_soldjers_per_round;
    }

    //exit condition
    if building_health == 0 && enemy_soldjers == 0 {
        current_round + 1
    } else {
        if my_surviving_soldjers <= 0 {
            i32::MAX
        } else {
            0
        }
    }
}

fn get_after_soldjer_attack(old_state: &State, state: &State, current_round: i32) -> i32 {
    0
}
