use std::fs;

fn part1(contents: &str) {
    println!("Part 1");
}

fn main() {
    let contents =
        fs::read_to_string("src/16/input.txt").expect("Should have been able to read the file");
    part1(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(2 + 2, 4);
    }
}
