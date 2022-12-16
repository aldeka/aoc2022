use core::panic;
use grid::Grid;
use std::cmp;
use std::fs;

type Point = (usize, usize);

type RockLine = Vec<Point>;

#[derive(Clone, Default)]
enum CavePixel {
    Rock,
    #[default]
    Empty,
    Sand,
}

type Cave = Grid<CavePixel>;

fn render_pixel(px: &CavePixel) -> char {
    match px {
        CavePixel::Rock => '#',
        CavePixel::Empty => '.',
        CavePixel::Sand => 'o',
    }
}

const SAND_START: Point = (500, 0);

fn next_sand_point(cave: &Cave, curr: Point) -> Point {
    let (mut x, mut y) = curr;
    match cave[x][y + 1] {
        CavePixel::Empty => y += 1,
        _ => match cave[x - 1][y + 1] {
            CavePixel::Empty => {
                x += 1;
                y += 1;
            }
            _ => match cave[x + 1][y + 1] {
                CavePixel::Empty => {
                    x += 1;
                    y += 1;
                }
                _ => {
                    return (x, y);
                }
            },
        },
    }
    (x, y)
}

fn add_sand(cave: &mut Cave) -> Point {
    let (mut x, mut y) = SAND_START;
    let mut has_stopped = false;
    while !has_stopped && y < cave.cols() - 1 {
        let next = next_sand_point(&cave, (x, y));
        if next.0 == x && next.1 == y {
            has_stopped = true;
        } else {
            x = next.0;
            y = next.1;
        }
    }

    if y < cave.rows() - 1 {
        cave[x][y] = CavePixel::Sand;
    } else {
        return (x, usize::MAX);
    }

    (x, y)
}

fn put_rocks_in_cave(cave: &mut Cave, lines: &Vec<RockLine>) {
    for line in lines {
        for i in 0..line.len() {
            let (x, y) = line[i];
            // this point is definitely a rock
            cave[x][y] = CavePixel::Rock;
            // now let's draw the line with the next point if there is one
            match line.get(i + 1) {
                Some((x1, y1)) => {
                    // draw the line segment between point a and point b
                    let diff_x: i32 = *x1 as i32 - x as i32;
                    let diff_y: i32 = *y1 as i32 - y as i32;
                    if diff_x != 0 {
                        // this is a horizontal line
                        let mut i = 0;
                        while i < diff_x.abs() {
                            let mut step: i32 = 1;
                            if diff_x < 0 {
                                step = -1;
                            }
                            cave[(x as i32 + step) as usize][y] = CavePixel::Rock;
                            i += 1;
                        }
                    } else if diff_y != 0 {
                        // vertical line
                        let mut i = 0;
                        while i < diff_y {
                            let mut step: i32 = 1;
                            if diff_x < 0 {
                                step = -1;
                            }
                            cave[x][(y as i32 + step) as usize] = CavePixel::Rock;
                            i += 1;
                        }
                    }
                }
                None => break,
            }
        }
    }
}

fn parse_rock_lines(data: &str) -> Vec<RockLine> {
    let rock_lines: Vec<RockLine> = data
        .trim()
        .split("\n")
        .map(|line| {
            line.split(" -> ")
                .map(|raw_point| {
                    let raw_coords: Vec<&str> = raw_point.split(",").collect();
                    if raw_coords.len() != 2 {
                        panic!("Wrong number of coordinates! {:#?}", raw_point);
                    }
                    return match (
                        raw_coords[0].parse::<usize>(),
                        raw_coords[1].parse::<usize>(),
                    ) {
                        (Ok(x), Ok(y)) => (x, y),
                        _ => panic!("Coordinates did not parse correctly: {:#?}", raw_coords),
                    };
                })
                .collect()
        })
        .collect();
    rock_lines
}

fn get_maxima(rock_lines: &Vec<RockLine>) -> (usize, usize) {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for line in rock_lines.iter() {
        for (x, y) in line.iter() {
            max_x = cmp::max(max_x, *x);
            max_y = cmp::max(max_y, *y);
        }
    }
    return (max_x, max_y);
}

fn part1(contents: &str) -> usize {
    println!("Part 1");

    let rock_lines = parse_rock_lines(contents);
    let (max_x, max_y) = get_maxima(&rock_lines);

    let mut cave = Grid::new(max_x + 1, max_y + 1);
    put_rocks_in_cave(&mut cave, &rock_lines);

    let mut sand_counter: usize = 0;
    let mut last_sand_added = SAND_START;
    while last_sand_added.1 != usize::MAX {
        last_sand_added = add_sand(&mut cave);
        sand_counter += 1;
    }

    sand_counter
}

fn main() {
    let contents =
        fs::read_to_string("src/14/input.txt").expect("Should have been able to read the file");
    part1(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_rock_lines() {
        let test_input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let result = parse_rock_lines(test_input);
        println!("{:#?}", result);
        assert_eq!(result.len(), 2);
        assert_eq!(result[1].len(), 4);
    }

    #[test]
    fn test_sand() {
        let mut cave: Grid<CavePixel> = Grid::new(503, 5);
        let sand_position = add_sand(&mut cave);
        assert_eq!(sand_position, (500, 4));
    }

    #[test]
    fn test_part1() {
        let test_input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

        let result = part1(test_input);
        println!("{:#?}", result);
        assert_eq!(result, 24);
    }
}
