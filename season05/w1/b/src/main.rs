use std::fs::File;
use std::io::BufRead;

fn main() {
    // Define the file path
    let file_path = "input.txt";

    // Open the file in read mode
    let file = File::open(file_path).expect("Error opening file");

    // Create a buffered reader
    let reader = std::io::BufReader::new(file);

    // Iterate over lines in the file
    let mut parameters: Vec<i32> = vec![];
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let line_components = line.split(':');
        for componet in line_components {
            parameters.push(componet.parse().unwrap());
        }
    }
    if parameters.len() == 5 {
        let current_game_state = GameState::new(
            parameters[0],
            parameters[1],
            parameters[2],
            parameters[3],
            parameters[4] == 2,
            0,
        );
        println!("{}", current_game_state.get_required_hits());
    } else {
        println!("something is wrong");
    }
}

struct GameState {
    first_match_hit: i32,
    first_match_miss: i32,
    second_match_hit: i32,
    second_match_midd: i32,
    is_second_match_home: bool,
    balls_to_hit: i32,
}

impl GameState {
    fn is_game_won(&self) -> bool {
        let delta = self.first_match_hit - self.first_match_miss + self.second_match_hit
            - self.second_match_midd;
        match delta {
            1..=std::i32::MAX => true,
            0 => self.is_game_won_for_equal_score(),
            std::i32::MIN..=-1 => false,
        }
    }
    fn get_required_hits(&self) -> i32 {
        if self.is_game_won() {
            self.balls_to_hit
        } else {
            let new_game_state = GameState::new(
                self.first_match_hit,
                self.first_match_miss,
                self.second_match_hit + 1,
                self.second_match_midd,
                self.is_second_match_home,
                self.balls_to_hit + 1,
            );
            new_game_state.get_required_hits()
        }
    }
    fn new(
        first_match_hit: i32,
        first_match_miss: i32,
        second_match_hit: i32,
        second_match_midd: i32,
        is_second_match_home: bool,
        balls_to_hit: i32,
    ) -> Self {
        Self {
            first_match_hit,
            first_match_miss,
            second_match_hit,
            second_match_midd,
            is_second_match_home,
            balls_to_hit,
        }
    }

    fn is_game_won_for_equal_score(&self) -> bool {
        let mut me_guest_match_hit = self.first_match_hit;
        let mut enemy_guest_match_hit = self.second_match_midd;

        if !self.is_second_match_home {
            me_guest_match_hit = self.second_match_hit;
            enemy_guest_match_hit = self.first_match_miss
        }

        me_guest_match_hit > enemy_guest_match_hit
    }
}
