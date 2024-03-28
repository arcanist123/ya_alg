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

    let number_of_rounds = get_number_of_rounds(
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
    if number_of_rounds == i32::MAX {
        println!("-1");
    } else {
        println!("{}", number_of_rounds);
    }
}

enum Actions {
    AttackSoldjers,
    AttackBuilding,
}

#[derive(PartialEq)]
struct State {
    my_soldjers: i32,
    building_health: i32,
    enemy_soldjers: i32,
    enemy_soldjers_per_round: i32,
}

impl State {
    fn apply_action(&self, action: &Actions) -> State {
        match action {
            Actions::AttackBuilding => {
                let my_unused_soldjers = self.my_soldjers - self.building_health;

                State {
                    my_soldjers: self.my_soldjers,
                    building_health: self.building_health - self.my_soldjers,
                    enemy_soldjers: self.enemy_soldjers,
                    enemy_soldjers_per_round: self.enemy_soldjers_per_round,
                }
            }
            Actions::AttackSoldjers => State {
                my_soldjers: self.my_soldjers,
                building_health: self.building_health,
                enemy_soldjers: self.enemy_soldjers,
                enemy_soldjers_per_round: self.enemy_soldjers_per_round,
            },
        }
    }
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
    } else if my_surviving_soldjers <= 0 || old_state == state {
        i32::MAX
    } else {
        get_number_of_rounds(
            state,
            &State {
                building_health,
                enemy_soldjers,
                enemy_soldjers_per_round: state.enemy_soldjers_per_round,
                my_soldjers: my_surviving_soldjers,
            },
            current_round + 1,
        )
    }
}

fn get_after_soldjer_attack(old_state: &State, state: &State, current_round: i32) -> i32 {
    //my move
    //first attack soldjers
    let mut remaining_enemy_soldjers = state.enemy_soldjers - state.my_soldjers;
    if remaining_enemy_soldjers < 0 {
        remaining_enemy_soldjers = 0
    }

    let mut my_remaining_soldjers = state.my_soldjers - state.enemy_soldjers;
    if my_remaining_soldjers < 0 {
        my_remaining_soldjers = 0
    }

    //now attack the building
    let building_health = state.building_health - my_remaining_soldjers;

    //enemy move
    // enemy attacks
    let my_surviving_soldjers = state.my_soldjers - remaining_enemy_soldjers;
    //building produces enemy soldjers
    if building_health > 0 {
        remaining_enemy_soldjers += state.enemy_soldjers_per_round;
    }

    if remaining_enemy_soldjers == 0 && building_health == 0 {
        current_round + 1
    } else if my_surviving_soldjers <= 0 || state == old_state {
        i32::MAX
    } else {
        get_number_of_rounds(
            state,
            &State {
                building_health,
                enemy_soldjers: remaining_enemy_soldjers,
                enemy_soldjers_per_round: state.enemy_soldjers_per_round,
                my_soldjers: my_surviving_soldjers,
            },
            current_round + 1,
        )
    }
}
