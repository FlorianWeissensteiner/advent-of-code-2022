use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut part1_total_score = 0;
    let mut part2_total_score = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (first_column, second_column) = line.split_once(' ').unwrap();

        {
            let opponent_shape = Shape::from(first_column);
            let my_shape = Shape::from(second_column);
            part1_total_score += get_shape_score(&my_shape);
            let outcome = calculate_outcome(&my_shape, &opponent_shape);
            part1_total_score += get_outcome_score(&outcome);
        }

        {
            let opponent_shape = Shape::from(first_column);
            let desired_outcome = Outcome::from(second_column);
            part2_total_score += get_outcome_score(&desired_outcome);
            let my_shape = match desired_outcome {
                Outcome::Victory => get_winning_shape(&opponent_shape),
                Outcome::Draw => get_drawing_shape(&opponent_shape),
                Outcome::Defeat => get_losing_shape(&opponent_shape),
            };
            part2_total_score += get_shape_score(&my_shape);
        }

    }
    println!("Part 1 total score: {}", part1_total_score);
    println!("Part 2 total score: {}", part2_total_score);
}

fn calculate_outcome(my_shape: &Shape, opponent_shape: &Shape) -> Outcome {
    if *my_shape == Shape::Rock && *opponent_shape == Shape::Scissors ||
       *my_shape == Shape::Paper && *opponent_shape == Shape::Rock ||
       *my_shape == Shape::Scissors && *opponent_shape == Shape::Paper {
        return Outcome::Victory;
    }
    if *my_shape == Shape::Rock && *opponent_shape == Shape::Paper ||
       *my_shape == Shape::Paper && *opponent_shape == Shape::Scissors ||
       *my_shape == Shape::Scissors && *opponent_shape == Shape::Rock {
        return Outcome::Defeat;
    }
    Outcome::Draw
}

fn get_winning_shape(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn get_losing_shape(shape: &Shape) -> Shape {
    match shape {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn get_drawing_shape(shape: &Shape) -> Shape {
    *shape
}

fn get_shape_score(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_outcome_score(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Victory => 6,
        Outcome::Draw => 3,
        Outcome::Defeat => 0,
    }
}

#[derive(Debug)]
enum Outcome {
    Victory,
    Draw,
    Defeat
}

impl From<&str> for Outcome {
    fn from(outcome_str: &str) -> Self {
        match outcome_str {
            "X" => Outcome::Defeat,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Victory,
            _ => panic!("Invalid Outcome parsed!")
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl From<&str> for Shape {
    fn from(shape_str: &str) -> Self {
        match shape_str {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("Invalid Shape parsed!")
        }
    }
}
