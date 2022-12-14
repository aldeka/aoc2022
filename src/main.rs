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

#[derive(Debug, PartialEq)]
struct MapPoints {
    start: Point,
    end: Point,
}

fn find_start_and_end(map: &Map) -> MapPoints {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    let mut iter = map.iter();
    let mut i = 0;
    while start == None || end == None {
        let next = iter.next();
        match next {
            Some(c) => match c.to_string().as_str() {
                "S" => start = Some((i % map.cols(), i % map.rows() - 1)),
                "E" => end = Some((i % map.cols(), i % map.rows() - 1)),
                _ => {}
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

fn next_step_candidates(current: Point) -> [Point; 4] {
    // let mut candidates = Vec::new();
    // candidates.push((current.0 + 1, current.1));
    // candidates.push((current.0, current.1 + 1));
    // if current.0 - 1 >= 0 {
    //     candidates.push((current.0 - 1, current.1));
    // }
    // if current.1 - 1 >= 0 {
    //     candidates.push((current.0, current.1 - 1));
    // }
    // candidates
    let mut min_x: usize = usize::MAX;
    match current.0.checked_sub(1) {
        Some(x) => min_x = x,
        None => {}
    }

    let mut min_y: usize = usize::MAX;
    match current.1.checked_sub(1) {
        None => {}
        Some(y) => min_y = y,
    }
    [
        (min_x, current.1),
        (current.0 + 1, current.1),
        (current.0, min_y),
        (current.0, current.1 + 1),
    ]
}

fn get_path_length(map: &Map) -> i32 {
    return map
        .iter()
        .filter(|c| c != &&'.')
        .collect::<Vec<&char>>()
        .len()
        .try_into()
        .unwrap();
}

fn can_step(a: char, b: char) -> Option<i32> {
    let mut digit_a: Option<u32> = a.to_digit(10);
    let mut digit_b: Option<u32> = b.to_digit(10);

    if digit_a != None && digit_b != None {
        if a == 'S' {
            digit_a = 'a'.to_digit(10);
        }
        if b == 'E' {
            digit_b = 'z'.to_digit(10);
        }
        let step_size = (digit_a.unwrap() as i32 - digit_b.unwrap() as i32).abs();
        if step_size == 1 || digit_a.unwrap() < digit_b.unwrap() {
            return Some(step_size);
        } //cmp::max(digit_a, digit_b) - cmp::min(digit_a, digit_b);
    }
    // if a == 'S' {
    //     digit_a = 'a'.to_digit(10).unwrap() as i32;
    // }
    // if b == 'E' {
    //     digit_b = 'z'.to_digit(10).unwrap() as i32;
    // }
    // let step_size = (digit_a - digit_b).abs();
    // if step_size == 1 || digit_a < digit_b {
    //     return Some(step_size);
    // } //cmp::max(digit_a, digit_b) - cmp::min(digit_a, digit_b);
    None
}

fn pathfinder(
    map: &mut Map,
    current: Point,
    end: Point,
    current_shortest: &mut i32,
) -> Option<Vec<Map>> {
    let current_value = *map.get(current.0, current.1).unwrap();
    map[current.0][current.1] = '.';
    let mut solutions = Vec::new();
    let path_length: i32 = get_path_length(map);
    if path_length >= *current_shortest {
        // this path will not win, nope out
        return None;
    }
    //if map.get(current.0, current.1).unwrap() == &'E' {
    if current == end {
        // hooray, we found a solution!
        println!("Found solution:\n{:#?}", map);
        solutions.push(map.clone());
        *current_shortest = path_length;
    } else {
        let next_steps = next_step_candidates(current);
        for step in next_steps {
            match map.get(step.0, step.1) {
                // is it a step small enough we can take it?
                Some(contents) => match can_step(*contents, current_value) {
                    Some(_) => {
                        let mut new_map = map.clone();
                        match &mut pathfinder(&mut new_map, step, end, current_shortest) {
                            Some(solution) => solutions.append(solution),
                            None => continue,
                        }
                    }
                    None => continue,
                },
                None => continue,
            }
        }
        return None;
    }
    Some(solutions)
}

fn part1(contents: &str) {
    println!("Part 1");
    let map = parse_map(contents);
    println!("Map:\n{:#?}", map);
    let start_and_end = find_start_and_end(&map);
    println!(
        "Start: {:#?} End: {:#?}",
        start_and_end.start, start_and_end.end
    );
    let mut mutable_map = map.clone();
    let mut current_shortest = i32::MAX;
    let solutions = pathfinder(
        &mut mutable_map,
        start_and_end.start,
        start_and_end.end,
        &mut current_shortest,
    );
    println!("Solutions:\n{:#?}", solutions);
}

fn main() {
    let contents =
        fs::read_to_string("src/12/input.txt").expect("Should have been able to read the file");
    part1(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let test_contents = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let map = parse_map(&test_contents);
        assert_eq!(
            find_start_and_end(&map),
            MapPoints {
                start: (0, 0),
                end: (5, 2),
            }
        );
    }
}
