use std::fs;
use std::collections::{VecDeque, HashSet};

fn is_unique_window(window: VecDeque<char>, window_len: usize) -> bool {
    let window_set: HashSet<char> = window.into_iter().collect();
    window_set.len() == window_len
}

fn part1() -> i32 {
    println!("Part 1");

    let window_length: usize = 4;

    let contents =
        fs::read_to_string("src/06/input.txt").expect("Should have been able to read the file");

    let mut window: VecDeque<char> = VecDeque::new();
    let chars = contents.chars();
    let mut index = 0;
    for char in chars {
        if is_unique_window(window.clone(),window_length) {
            println!("Index: {index}");
            return index;
        }
        if window.len() == window_length {
            window.pop_front();
        } else if window.len() > window_length {
            panic!("Window got too big!");
        }
        window.push_back(char);
        println!("Window:\n{window:#?}");
        index += 1;
    }
    println!("Failed to find window: {index}");
    return -1
}

fn part2() -> usize {
    println!("Part 2");

    let window_length: usize = 14;

    let contents =
        fs::read_to_string("src/06/input.txt").expect("Should have been able to read the file");

    let chars: Vec<char> = contents.chars().collect();
    let chars_iter = chars.windows(window_length);
    let mut index = 0;
    for window in chars_iter {
        let is_unique_window: bool = HashSet::<&char>::from_iter(window.iter()).len() == window_length;
        if is_unique_window {
            println!("Index: {}", index + window_length);
            break;
        }
        index += 1;
    }
    index + window_length
}

fn main() {
    part1();
    part2();
}
