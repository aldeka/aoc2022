use std::fs;
use std::collections::HashSet;

fn find_start_of_message(contents: &str, window_length: usize) -> usize {
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

fn part1(contents: &str) {
    println!("Part 1");

    let window_length: usize = 4;
    find_start_of_message(contents, window_length);
}

fn part2(contents: &str) {
    println!("Part 2");

    let window_length: usize = 14;
    find_start_of_message(contents, window_length);
}

fn main() {
    let contents =
        fs::read_to_string("src/06/input.txt").expect("Should have been able to read the file");
    part1(&contents);
    part2(&contents);
}

#[cfg(test)]
mod tests {
    use crate::find_start_of_message;

    #[test]
    fn my_great_test() {
        let sample_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_start_of_message(sample_input, 14), 19);
        println!("hello");
    }
}