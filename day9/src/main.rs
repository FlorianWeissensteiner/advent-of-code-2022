use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashSet;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut short_rope: Vec<Knot> = vec![Knot::new(), Knot::new()];

    let mut long_rope: Vec<Knot> = Vec::new();
    for _ in 0..10 {
        long_rope.push(Knot::new());
    }

    for line in reader.lines() {
        let line = line.unwrap();
        let split_line: Vec<&str> = line.split_whitespace().collect();

        let direction = Direction::from(split_line[0]);
        let moves = u32::from_str(split_line[1]).unwrap();

        for _ in 0..moves {
            simulate_rope(&mut short_rope, &direction);
            simulate_rope(&mut long_rope, &direction);
        }
    }

    println!("Short rope tail visited {} positions.", short_rope.last().unwrap().positions_visited.len());
    println!("Long rope tail visited {} positions.", long_rope.last().unwrap().positions_visited.len());
}

fn simulate_rope(rope: &mut Vec<Knot>, direction: &Direction) {
    for i in 0..rope.len() {
        if i == 0 {
            rope[0].move_in_direction(direction);
            continue;
        }
        let (first_half, second_half) = rope.split_at_mut(i);
        let previous_knot = first_half.last().unwrap();
        let current_knot = second_half.first_mut().unwrap();
        current_knot.move_towards_knot(previous_knot);
    }
}

struct Knot {
    pub position: Position,
    pub positions_visited: HashSet<Position>,
}

impl Knot {
    pub fn new() -> Self{
        let mut positions_visited = HashSet::new();
        positions_visited.insert(Position{ x: 0, y: 0 });
        Self {
            position: Position{ x: 0, y: 0 },
            positions_visited,
        }
    }

    pub fn move_in_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Right => self.position.x += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
        }
        self.positions_visited.insert(self.position.clone());
    }

    pub fn move_towards_knot(&mut self, target: &Knot) {
       if !self.is_touching(target) {
           let x_distance = self.position.x - target.position.x;
           match x_distance.cmp(&0) {
               Ordering::Greater => self.position.x -= 1,
               Ordering::Less => self.position.x += 1,
               Ordering::Equal => (),
           }

           let y_distance = self.position.y - target.position.y;
           match y_distance.cmp(&0) {
               Ordering::Greater => self.position.y -= 1,
               Ordering::Less => self.position.y += 1,
               Ordering::Equal => (),
           }

           self.positions_visited.insert(self.position.clone());
       }
    }

    fn is_touching(&self, target: &Knot) -> bool {
         (self.position.x - target.position.x).abs() <= 1 && (self.position.y - target.position.y).abs() <= 1
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Up, Right, Down, Left,
}

impl From<&str> for Direction {
    fn from(direction_str: &str) -> Self {
        match direction_str {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => Direction::Down
        }
    }
}
