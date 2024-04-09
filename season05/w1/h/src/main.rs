fn main() {
    let state = RunState::new(0f64, 0f64, 0f64, 0f64, 0f64);

    println!("{}", state.get_time());
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
        Self {
            speed1: self.speed1,
            position1: 0f64,
            speed2: self.speed2,
            position2: 0f64,
            time: 0f64,
        }
    }
    fn new(speed1: f64, position1: f64, speed2: f64, position2: f64, time: f64) -> Self {
        Self {
            speed1: if position1 < 0f64 {
                0f64 - speed1
            } else {
                speed1
            },
            position1: {
                if position1 < 0f64 {
                    0f64 - position1
                } else {
                    position1
                }
            },
            speed2: if position2 < 0f64 {
                0f64 - speed2
            } else {
                speed2
            },
            position2: if position2 < 0f64 {
                0f64 - position2
            } else {
                position2
            },
            time,
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
        if self.speed1 < 0f64 {
            self.inverse_state().get_time();
        } else {
            let next_state = self.transition_state();
            if RunState::is_intervals_intersect(
                RunState::get_number_at_precision(self.position1),
                RunState::get_number_at_precision(next_state.position1),
                RunState::get_number_at_precision(self.position2),
                RunState::get_number_at_precision(next_state.position2),
            ) {
                if self.speed2 > 0f64 {
                } else {
                }
            }
        }

        0f64
    }
    fn get_number_at_precision(value: f64) -> i32 {
        (value * 10000000000f64) as i32
    }
    fn is_intervals_intersect(start_1: i32, end_1: i32, start_2: i32, end_2: i32) -> bool {
        core::cmp::max(start_1, start_2) > core::cmp::min(end_1, end_2)
    }
}
