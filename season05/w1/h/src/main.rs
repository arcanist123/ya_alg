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

    let mut length = 0;
    let mut speed1 = 0;
    let mut position1 = 0;
    let mut speed2 = 0;
    let mut position2 = 0;
    for first_line in lines.iter().take(1) {
        let components = first_line.split(' ');
        for (position, component) in components.take(5).enumerate() {
            match position {
                0 => length = component.parse().unwrap(),
                1 => position1 = component.parse().unwrap(),
                2 => speed1 = component.parse().unwrap(),
                3 => position2 = component.parse().unwrap(),
                4 => speed2 = component.parse().unwrap(),
                _ => (),
            }
        }
    }
    let time: f64;
    if speed1 == 0 && speed2 == 0 {
        if position1 == position2 {
            time = 0f64;
        } else {
            time = -1f64;
        }
    } else if speed1 == 0 {
        time = RunState::create_from_i32(speed2, position2, speed1, position1, length).get_time();
    } else {
        time = RunState::create_from_i32(speed1, position1, speed2, position2, length).get_time();
    }

    // let state = RunState::new(0f64, 0f64, 0f64, 0f64, 0f64);

    println!("{}", time);
}

#[derive(Debug)]
struct RunState {
    speed1: f64,
    position1: f64,
    speed2: f64,
    position2: f64,
    time: f64,
}

impl RunState {
    fn transition_state(&self) -> Self {
        let time = match self.speed2 {
            x if x < 0f64 => f64::min(
                (1f64 - self.position1) / self.speed1,
                self.position2 / self.speed2,
            ),
            x if x == 0f64 => (1f64 - self.position1) / self.speed1,
            x if x > 0f64 => f64::min(
                (1f64 - self.position1) / self.speed1,
                (1f64 - self.position2) / self.speed2,
            ),
            _ => 2f64,
        };

        Self {
            speed1: self.speed1,
            position1: self.position1 + (self.speed1 * time),
            speed2: self.speed2,
            position2: self.position2 + (self.speed2 * time),
            time: self.time + time,
        }
    }
    fn get_relative_position_speed(length: i32, position: i32, speed: i32) -> (f64, f64) {
        let relative_position = position as f64 / (length as f64 / 2f64);
        let relative_speed = speed as f64 / (length as f64 / 2f64);
        if relative_position > 1f64 {
            (1f64 - relative_position, relative_speed * -1f64)
        } else {
            (relative_position, relative_speed)
        }
    }
    fn create_from_i32(
        speed1: i32,
        position1: i32,
        speed2: i32,
        position2: i32,
        length: i32,
    ) -> Self {
        let (x1, v1) = RunState::get_relative_position_speed(length, position1, speed1);
        let (x2, v2) = RunState::get_relative_position_speed(length, position2, speed2);
        RunState::new(v1, x1, v2, x2, 0f64)
    }
    fn new(speed1: f64, position1: f64, speed2: f64, position2: f64, time: f64) -> Self {
        let state = Self {
            speed1,
            position1,
            speed2,
            position2,
            time,
        };
        if state.speed1 < 0f64 {
            state.inverse_state()
        } else {
            state
        }
    }
    fn inverse_state(&self) -> Self {
        Self {
            speed1: 0f64 - self.speed1,
            position1: 1f64 - self.position1,
            speed2: 0f64 - self.speed2,
            position2: 1f64 - self.position2,
            time: self.time,
        }
    }
    fn get_time(&self) -> f64 {
        let next_state = self.transition_state();
        let interval_intersect = RunState::is_intervals_intersect(
            RunState::get_number_at_precision(self.position1),
            RunState::get_number_at_precision(next_state.position1),
            RunState::get_number_at_precision(self.position2),
            RunState::get_number_at_precision(next_state.position2),
        );
        if interval_intersect {
            if self.speed2 > 0f64 {
                (self.position2 - self.position1) / (self.speed1 - self.speed2)
            } else {
                let mut delta_position = self.position1 - self.position2;
                if delta_position < 0f64 {
                    delta_position = 0f64 - delta_position;
                }
                delta_position / (self.speed1 + self.speed2)
            }
        } else {
            next_state.get_time()
        }
    }
    fn get_number_at_precision(value: f64) -> i64 {
        (value * 10000000000f64) as i64
    }
    fn is_intervals_intersect(start_1: i64, end_1: i64, start_2: i64, end_2: i64) -> bool {
        if start_1 <= start_2 {
            start_2 <= end_1
        } else {
            start_1 <= end_2
        }
        // core::cmp::max(start_1, start_2) >= core::cmp::min(end_1, end_2)
    }
}
