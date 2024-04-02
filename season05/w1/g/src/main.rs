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

    // for a in 4..10 {
    //     println!("{}", a);
    //     for b in 4990..4995 {
    //         // if a == 3 {
    //         //     println!("{}", b);
    //         // }
    //         for c in 1..5000 {
    //             let solution = get_number_of_rounds(
    //                 &State {
    //                     my_soldjers: a,
    //                     building_health: b,
    //                     enemy_soldjers: 0,
    //                     enemy_soldjers_per_round: c,
    //                 },
    //                 1,
    //             );

    //             println!("the solution for {} {} {} is {}", a, b, c, solution);
    //         }
    //     }
    // }

    let number_of_rounds = get_number_of_rounds(
        &State {
            my_soldjers,
            building_health,
            enemy_soldjers: 0,
            enemy_soldjers_per_round,
        },
        1,
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

#[derive(PartialEq, Debug)]
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
                let remaining_building_health = (self.building_health - self.my_soldjers).max(0);
                let my_unused_soldjers = (self.my_soldjers - self.building_health).max(0);
                let enemy_soldjers = (self.enemy_soldjers - my_unused_soldjers).max(0);
                let my_remaining_soldjers = (self.my_soldjers - enemy_soldjers).max(0);
                let mut enemy_soldjers_after_respawn = enemy_soldjers;
                if remaining_building_health > 0 {
                    enemy_soldjers_after_respawn += self.enemy_soldjers_per_round;
                }
                State {
                    my_soldjers: my_remaining_soldjers,
                    building_health: remaining_building_health,
                    enemy_soldjers: enemy_soldjers_after_respawn,
                    enemy_soldjers_per_round: self.enemy_soldjers_per_round,
                }
            }
            Actions::AttackSoldjers => {
                let remaining_enemy_soldjers = (self.enemy_soldjers - self.my_soldjers).max(0);
                let my_unused_soldjers = (self.my_soldjers - self.enemy_soldjers).max(0);
                let remaining_building_health = (self.building_health - my_unused_soldjers).max(0);
                let my_remaining_soldjers = (self.my_soldjers - remaining_enemy_soldjers).max(0);
                let mut enemy_soldjers_after_respawn = remaining_enemy_soldjers;
                if remaining_building_health > 0 {
                    enemy_soldjers_after_respawn += self.enemy_soldjers_per_round;
                }

                State {
                    my_soldjers: my_remaining_soldjers,
                    building_health: remaining_building_health,
                    enemy_soldjers: enemy_soldjers_after_respawn,
                    enemy_soldjers_per_round: self.enemy_soldjers_per_round,
                }
            }
        }
    }
}

fn get_number_of_rounds(state: &State, current_round: i32) -> i32 {
    i32::min(
        get_after_building_attack(state, current_round),
        get_after_soldjer_attack(state, current_round),
    )
}

fn get_after_building_attack(state: &State, current_round: i32) -> i32 {
    println!("{:?}", state);
    let next_state = state.apply_action(&Actions::AttackBuilding);
    if &next_state == state || next_state.my_soldjers == 0 || current_round > 10000 {
        i32::MAX
    } else if next_state.enemy_soldjers == 0 && next_state.building_health == 0 {
        current_round
    } else {
        get_number_of_rounds(&next_state, current_round + 1)
    }
}

fn get_after_soldjer_attack(state: &State, current_round: i32) -> i32 {
    let next_state = state.apply_action(&Actions::AttackSoldjers);
    if &next_state == state || next_state.my_soldjers == 0 || current_round > 10000 {
        i32::MAX
    } else if next_state.enemy_soldjers == 0 && next_state.building_health == 0 {
        current_round
    } else {
        get_number_of_rounds(&next_state, current_round + 1)
    }
}
