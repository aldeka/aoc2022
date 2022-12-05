use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/01/input.txt")
        .expect("Should have been able to read the file");

    let mut split = contents.split("\n");
    let mut elves = Vec::new();
    let mut current_elf_calories = 0;
    for line in split {
        if line == "" {
            elves.push(currentElfCalories);
            current_elf_calories = 0;
        } else {
          current_elf_calories += line.parse::<i32>().unwrap();
        }
    }
    elves.sort();
    let mut sum = 0;
    for elf in &elves[elves.len() - 3..] {
        sum += elf;
    }
    println!("Top three elves' sum:\n{sum}");
}