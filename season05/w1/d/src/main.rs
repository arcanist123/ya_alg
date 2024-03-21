use std::collections::{HashMap, HashSet};
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

    let mut points: HashMap<Point, CellFigure> = HashMap::new();

    for (current_x, line) in lines.iter().take(8).enumerate() {
        for (current_y, symbol) in line.chars().take(8).enumerate() {
            points.insert(
                Point::new(current_x as i32, current_y as i32),
                match symbol {
                    'R' => CellFigure::Rook,
                    'B' => CellFigure::Bishop,
                    _ => CellFigure::EmptyCell,
                },
            );
        }
    }
    let mut hit_cells: HashSet<Point> = HashSet::new();
    for (point, figure) in points.iter() {
        if figure != &CellFigure::EmptyCell {
            hit_cells.extend(point.get_hit_cells(&points));
        }
    }
    let free_cells = 64 - hit_cells.len();
    println!("{}", free_cells);
}
#[derive(Hash, Eq, PartialEq)]

enum CellFigure {
    EmptyCell,
    Bishop,
    Rook,
}
enum Directions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_hit_cells(&self, board: &HashMap<Point, CellFigure>) -> Vec<Point> {
        let current_cell = board.get(self).unwrap();
        let directions: Vec<Directions> = match current_cell {
            CellFigure::EmptyCell => Vec::new(),
            CellFigure::Bishop => vec![
                Directions::UpLeft,
                Directions::UpRight,
                Directions::DownRight,
                Directions::DownLeft,
            ],
            CellFigure::Rook => vec![
                Directions::Up,
                Directions::Down,
                Directions::Left,
                Directions::Right,
            ],
        };

        let mut hit_points: Vec<Point> = Vec::new();
        if current_cell != &CellFigure::EmptyCell {
            hit_points.push(Point::new(self.x, self.y))
        }
        for direction in directions {
            let direction_point = self.get_next_point(&direction);
            if board.contains_key(&direction_point)
                && board.get(&direction_point).unwrap() == &CellFigure::EmptyCell
            {
                hit_points.extend(direction_point.get_hit_cells_by_direction(board, &direction));
            }
        }
        hit_points
    }

    fn get_hit_cells_by_direction(
        &self,
        board: &HashMap<Point, CellFigure>,
        direction: &Directions,
    ) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();
        result.push(Point::new(self.x, self.y));
        let next_point: Point = self.get_next_point(direction);
        if board.contains_key(&next_point)
            && board.get(&next_point).unwrap() == &CellFigure::EmptyCell
        {
            result.extend(next_point.get_hit_cells_by_direction(board, direction));
        }

        result
    }

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn get_next_point(&self, direction: &Directions) -> Point {
        match direction {
            Directions::Down => Point::new(self.x, self.y + 1),
            Directions::Up => Point::new(self.x, self.y - 1),
            Directions::Left => Point::new(self.x - 1, self.y),
            Directions::Right => Point::new(self.x + 1, self.y),
            Directions::UpLeft => Point::new(self.x - 1, self.y - 1),
            Directions::UpRight => Point::new(self.x + 1, self.y - 1),
            Directions::DownLeft => Point::new(self.x - 1, self.y + 1),
            Directions::DownRight => Point::new(self.x + 1, self.y + 1),
        }
    }
}
