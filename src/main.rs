use std::{collections::HashSet, fs};

type Position = (i32, i32);

fn next_knot_position(head: Position, tail: Position) -> Position {
    let mut next_position = (tail.0, tail.1);
    let mut horiz_move = false;
    if (head.0 - tail.0).abs() > 1 {
        horiz_move = true;
        next_position.0 += (head.0 - tail.0).signum();
    }
    let mut vert_move = false;
    if (head.1 - tail.1).abs() > 1 {
        vert_move = true;
        next_position.1 += (head.1 - tail.1).signum();
    }

    if horiz_move && next_position.1 != head.1 {
        // shift vert too, because diagonal move
        next_position.1 += (head.1 - tail.1).signum();
    } else if vert_move && next_position.0 != head.0 {
        // shift horiz too, because diagonal move
        next_position.0 += (head.0 - tail.0).signum();
    }

    next_position
}

fn simulate_rope(contents: &str, knot_count: u32) -> usize {
    let mut visited_positions = HashSet::<Position>::new();
    let mut knots = Vec::<Position>::new();

    let mut i = 0;
    // initialize knot list
    while i < knot_count {
        knots.push((0, 0));
        i += 1;
    }

    visited_positions.insert(knots[knots.len() - 1].clone()); // because we start there

    for line in contents.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 2 {
            let direction = parts[0];
            let mut count = parts[1].parse::<i32>().unwrap();
            // println!("Move: {} {}", direction, count);

            while count > 0 {
                // move the head of the rope
                match direction {
                    "L" => knots[0].0 -= 1,
                    "R" => knots[0].0 += 1,
                    "D" => knots[0].1 -= 1,
                    "U" => knots[0].1 += 1,
                    _ => panic!("Unrecognized direction {direction}"),
                }
                // then move the subsequent knots one at a time
                let mut i = 1;
                while i < knots.len() {
                    knots[i] = next_knot_position(knots[i - 1], knots[i]);
                    i += 1;
                }
                // and record the tail position
                visited_positions.insert(knots[knots.len() - 1].clone());
                count -= 1;
            }
        }
    }
    // println!("{:#?}", visited_positions);
    visited_positions.len()
}

fn part1(contents: &str) {
    println!("Part 1");
    println!("Tail visitation count: {}", simulate_rope(contents, 2));
}

fn part2(contents: &str) {
    println!("Part 2");
    println!("Tail visitation count: {}", simulate_rope(contents, 10));
}

fn main() {
    let contents =
        fs::read_to_string("src/09/input.txt").expect("Should have been able to read the file");
    part1(&contents);
    println!("");
    part2(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_next_knot_position() {
        // up
        let head: Position = (0, 2);
        let tail: Position = (0, 0);
        let mut next: Position = next_knot_position(head, tail);
        assert_eq!(next, (0, 1));

        // down
        let head: Position = (0, -3);
        let tail: Position = (0, -1);
        next = next_knot_position(head, tail);
        assert_eq!(next, (0, -2));

        // right
        let head: Position = (0, 0);
        let tail: Position = (-3, 0);
        next = next_knot_position(head, tail);
        assert_eq!(next, (-2, 0));

        // left
        let head: Position = (3, 0);
        let tail: Position = (5, 0);
        next = next_knot_position(head, tail);
        assert_eq!(next, (4, 0));

        // diagonal
        let head: Position = (2, -1);
        let tail: Position = (0, 0);
        next = next_knot_position(head, tail);
        assert_eq!(next, (1, -1));

        // another diagonal
        let head: Position = (2, 3);
        let tail: Position = (1, 1);
        next = next_knot_position(head, tail);
        assert_eq!(next, (2, 2));

        // another diagonal
        let head: Position = (4, 1);
        let tail: Position = (3, 0);
        next = next_knot_position(head, tail);
        assert_eq!(next, (3, 0));
        // another diagonal
        let head: Position = (4, 2);
        let tail: Position = (3, 0);
        next = next_knot_position(head, tail);
        assert_eq!(next, (4, 1));
    }

    #[test]
    fn test_simulate_rope() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let visited_positions = simulate_rope(input, 2);
        assert_eq!(visited_positions, 13);
    }

    #[test]
    fn test_basic_long_rope() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let visited_positions = simulate_rope(input, 10);
        assert_eq!(visited_positions, 1);
    }

    #[test]
    fn test_advanced_long_rope() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let visited_positions = simulate_rope(input, 10);
        assert_eq!(visited_positions, 36);
    }

    #[test]
    fn test_hashset() {
        let mut hs = HashSet::<Position>::new();
        let blah: Position = (0, 1);
        hs.insert(blah.clone());
        hs.insert(blah.clone());
        hs.insert((1, 3));
        hs.insert((0, 1));
        hs.insert((3, 1));
        assert_eq!(hs.len(), 3);
    }
}
