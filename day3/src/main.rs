use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut compartment_priority_sum = 0;
    let mut badge_priority_sum = 0;

    let input_lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    for (idx, line) in input_lines.iter().enumerate() {
        let (first_half, second_half) = line.split_at(line.len() / 2);
        compartment_priority_sum += get_priority(char_in_common(first_half, second_half));

        if idx != 0 && (idx + 1) % 3 == 0 {
            badge_priority_sum += get_priority(char_in_common(&input_lines[idx], &chars_in_common(&input_lines[idx - 1], &input_lines[idx - 2])));
        }

    }

    println!("{}", compartment_priority_sum);
    println!("{}", badge_priority_sum);
}

fn char_in_common(str_a: &str, str_b: &str) -> char {
    for c in str_a.chars() {
        if str_b.contains(c) {
            return c;
        }
    }
    panic!("No character in common!");
}

fn chars_in_common(str_a: &str, str_b: &str) -> String {
    let mut chars = String::new();
    for c in str_a.chars() {
        if str_b.contains(c) {
            chars.push(c);
        }
    }
    chars
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        (c as u32) - 96 
    } else {
        (c as u32) - 38 
    }
}
