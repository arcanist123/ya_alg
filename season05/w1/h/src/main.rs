fn main() {
    let state = RunState::new(0, 0f64, 0, 0f64, 0f64);
    println!("{:?}", state);
}

#[derive(Debug)]
struct RunState {
    speed1: i32,
    position1: f64,
    speed2: i32,
    position2: f64,
    time: f64,
}

impl RunState {
    fn new(speed1: i32, position1: f64, speed2: i32, position2: f64, time: f64) -> Self {
        Self {
            speed1: if position1 < 0f64 { 0 - speed1 } else { speed1 },
            position1: {
                if position1 < 0f64 {
                    0f64 - position1
                } else {
                    position1
                }
            },
            speed2: if position2 < 0f64 { 0 - speed2 } else { speed2 },
            position2: if position2 < 0f64 {
                0f64 - position2
            } else {
                position2
            },
            time,
        }
    }
    fn get_time(&self, length: i32) -> f64 {
        let meeting_position =
            (self.position2 - self.position1) / (self.speed1 - self.speed2) as f64;
            if meeting_position 
        0f64
    }
}
