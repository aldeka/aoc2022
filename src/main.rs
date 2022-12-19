use std::fs;

fn part1(contents: &str) {
    println!("Part 1");

    let answer = "";
    println!("Answer: {}", answer);
}

fn main() {
    let contents =
        fs::read_to_string("src/18/input.txt").expect("Should have been able to read the file");
    part1(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        assert_eq!(2 + 2, 4)
    }
}
