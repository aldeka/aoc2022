use std::env;
use std::fs;

fn main() {
    // --snip--
    println!("Hello");

    let contents = fs::read_to_string("src/01/input.txt")
        .expect("Should have been able to read the file");

    let mut split = contents.split("\n");
    let mut elves = Vec::new();
    let mut current_elf_calories = 0;
    for line in split {
        if line == "" {
            elves.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
          current_elf_calories += line.parse::<i32>().unwrap();
        }
    }
    let max = elves.iter().max().expect("There should be a max value").to_string();

    println!("With text:\n{contents}");
    println!("Max:\n{max}");
}