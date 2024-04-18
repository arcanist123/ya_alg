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
            time = f64::MAX;
        }
    } else {
        time = RunState::create_from_i32(speed1, position1, speed2, position2, length).get_time();
    }

    if time == f64::MAX {
        println!("NO");
    } else {
        println!("YES");
        let formatted_string = format!("{:.10}", time);
        println!("{}", formatted_string)
    }
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
        let time1 = RunState::get_time_until_border(self.speed1, self.position1);
        let time2 = RunState::get_time_until_border(self.speed2, self.position2);
        let time = f64::min(time1, time2);
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
            (1f64 - (relative_position - 1f64), 0f64 - relative_speed)
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
        Self {
            speed1,
            position1,
            speed2,
            position2,
            time,
        }
    }

    fn inverse_state(&self) -> Self {
        Self {
            speed1: if RunState::is_at_border(self.position1) {
                0f64 - self.speed1
            } else {
                self.speed1
            },
            position1: self.position1,
            speed2: if RunState::is_at_border(self.position2) {
                0f64 - self.speed2
            } else {
                self.speed2
            },
            position2: self.position2,
            time: self.time,
        }
    }
    fn get_time(&self) -> f64 {
        // println!("current state is {:?}", self);
        let next_state = self.transition_state();
        // println!("next state is {:?}", next_state);

        let estimated_time = self.calculate_time();
        // println!(
        //     "estimated time is {}, state delta is {}",
        //     estimated_time,
        //     (next_state.time - self.time)
        // );
        if estimated_time == f64::MAX
            || estimated_time < 0f64
            || estimated_time > (next_state.time - self.time)
        {
            next_state.inverse_state().get_time()
        } else {
            self.time + estimated_time
        }
    }
    fn get_number_at_precision(value: f64) -> i64 {
        (value * 10000000000f64) as i64
    }
    fn is_at_border(position: f64) -> bool {
        RunState::get_number_at_precision(position) == 0
            || RunState::get_number_at_precision(position) == 10000000000
    }

    fn get_time_until_border(speed: f64, position: f64) -> f64 {
        match speed {
            x if x < 0f64 => (0f64 - position) / speed,
            x if x == 0f64 => f64::MAX,
            x if x > 0f64 => (1f64 - position) / speed,
            _ => f64::MAX,
        }
    }

    fn calculate_time(&self) -> f64 {
        if RunState::get_number_at_precision(self.position1)
            == RunState::get_number_at_precision(self.position2)
        {
            self.time + 0f64
        } else {
            //  speed1*time + position1 = speed2*time + position2
            //  (speed1 - speed2)*time = position2 - position1
            let time = (self.position2 - self.position1) / (self.speed1 - self.speed2);
            if time < 0f64 {
                f64::MAX
            } else {
                time
            }
        }
    }
}
