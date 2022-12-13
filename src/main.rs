use grid::Grid;
use std::fs;

type Map = Grid<char>;

fn parse_map(contents: &str) -> Grid<char> {
    let row_data: Vec<Vec<char>> = contents
        .trim()
        .split("\n")
        .map(|s| return s.chars().collect())
        .collect();
    let cols = row_data[0].len();
    let map: Map = Grid::from_vec(row_data.concat(), cols);
    map
}

type Point = (usize, usize);

struct MapPoints {
    start: Point,
    end: Point,
}

fn find_start_and_end(map: Map) -> MapPoints {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    let mut iter = map.iter();
    let mut i = 0;
    while (start == None || end == None) {
        let next = iter.next();
        match next {
            Some(c) => match c.to_string().as_str() {
                "S" => start = Some((i / map.cols(), i % map.rows())),
                "E" => end = Some((i / map.cols(), i % map.rows())),
                _ => continue,
            },
            None => panic!("No map element found here at index {i}"),
        }
        i += 1;
    }
    return MapPoints {
        start: start.unwrap(),
        end: end.unwrap(),
    };
}

fn get_char(s: &str) -> char {
    s.chars().collect::<Vec<char>>()[0]
}

fn next_step_candidates(current: Point) -> [Point; 4] {
    [
        (current.0 - 1, current.1),
        (current.0 + 1, current.1),
        (current.0, current.1 - 1),
        (current.0, current.1 + 1),
    ]
}

fn get_path_length(map: &Map) -> i32 {
    return map
        .iter()
        .filter(|c| c != &&get_char("."))
        .collect::<Vec<&char>>()
        .len()
        .try_into()
        .unwrap();
}

fn can_step(a: char, b: char) {
    let mut adjusted_a = a;
    let mut adjusted_b = b;
    if a == get_char("S") {
        a = get_char("a");
    }
    if b == get_char("E") {
        b = get_char("z");
    }
}

fn pathfinder(map: &mut Map, current: Point, current_shortest: &mut i32) -> Option<Vec<Map>> {
    let current_value = *map.get(current.0, current.1).unwrap();
    map[current.0][current.1] = get_char(".");
    let mut subpaths = Vec::new();
    let path_length: i32 = get_path_length(map);
    if path_length >= *current_shortest {
        // this path will not win, nope out
        return None;
    }
    if map.get(current.0, current.1).unwrap() == &get_char("E") {
        // hooray, we found a solution!
        subpaths.push(map.clone());
        current_shortest = *path_length;
    } else {
        let next_steps = next_step_candidates(current);
        for step in next_steps {
            match map.get(step.0, step.1) {
                // is it a step small enough we can take it?
                Some(contents) => match contents.to_string().as_str() {
                    "." => continue,
                },
                None => continue,
            }
        }
        return None;
    }
    Some(subpaths)
}

fn part1(contents: &str) {
    println!("Part 1");
    let map = parse_map(contents);
    let start_and_end = find_start_and_end(map);
}

fn main() {
    let contents =
        fs::read_to_string("src/12/input.txt").expect("Should have been able to read the file");
    part1(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    // #[test]
    // fn test_part1() {
    //     let contents =
    //     fs::read_to_string("src/11/test_input.txt").expect("Should have been able to read the file");
    //     part1(&contents);
    //     assert_eq!(2+2, 4);
    // }
}
