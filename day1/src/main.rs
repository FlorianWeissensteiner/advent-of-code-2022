use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::FromStr;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut summed_calories = Vec::new();
    let mut current_calories = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            summed_calories.push(current_calories);
            current_calories = 0;
            continue;
        }
        current_calories += u32::from_str(&line).unwrap();
    }
    summed_calories.sort();
    println!("The top Elf is carrying {} calories.", summed_calories.last().unwrap());
    println!("The top three Elves are carrying {} calories.", summed_calories[summed_calories.len()-1] + summed_calories[summed_calories.len()-2] + summed_calories[summed_calories.len()-3]);
}
