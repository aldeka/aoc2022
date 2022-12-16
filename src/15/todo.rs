extern crate gcollections;
extern crate interval;
use gcollections::ops::*;
use interval::ops::*;
use interval::Interval;
use regex::Regex;
use std::fs;

type Point = (i32, i32);

struct Sensor {
    loc: Point,
    distance: i32,
}

fn find_impossible_coords(y_val: i32, s: Sensor) -> Option<Interval<i32>> {
    let max_impossible_y = s.loc.1 + s.distance;
    let min_impossible_y = s.loc.1 - s.distance;
    if s.loc.1 < y_val && max_impossible_y >= y_val {
        // overlap below
        let overlap_amount = max_impossible_y - y_val;
        return Some(Interval::new(
            s.loc.0 - overlap_amount,
            s.loc.0 + overlap_amount + 1,
        ));
    } else if s.loc.1 > y_val && min_impossible_y <= y_val {
        // overlap above
        let overlap_amount = y_val - min_impossible_y;
        return Some(Interval::new(
            s.loc.0 - overlap_amount,
            s.loc.0 + overlap_amount + 1,
        ));
    }
    // no overlap
    None
}

fn find_coords(line: &str) -> Point {
    let re = Regex::new(r"x=(\d+), y=(\d+)").unwrap();
    let cap = re.captures(line).unwrap();
    (
        cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
        cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
    )
}

fn get_distance(a: &Point, b: &Point) -> i32 {
    let diff_x = (a.0 - b.0).abs();
    let diff_y = (a.1 - b.1).abs();

    diff_x + diff_y
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = Vec::new();
    for line in input.trim().split('\n') {
        let parts: Vec<&str> = line.split(": ").collect();
        let sensor_coords = find_coords(parts[0]);
        let raw_beacon_info = find_coords(parts[1]);
        let distance = get_distance(&sensor_coords, &raw_beacon_info);
        sensors.push(Sensor {
            loc: sensor_coords,
            distance,
        });
    }
    sensors
}

fn part1(contents: &str) {
    println!("Part 1");

    const TESTED_Y_VAL: i32 = 200000;

    let sensors = parse_input(contents);
    // let mut impossible_coordinate_ranges: Vec<Interval<i32>> = Vec::new();
    let mut impossible_coordinate_ranges: Vec<Interval<i32>> = Vec::new();
    for sensor in sensors {
        match find_impossible_coords(TESTED_Y_VAL, sensor) {
            Some(x) => impossible_coordinate_ranges.push(x),
            None => continue,
        }
    }
    let range_hull = &impossible_coordinate_ranges
        .into_iter()
        .reduce(|r, arr| arr.hull(&r))
        .unwrap();

    for r in &impossible_coordinate_ranges {
        // println!("{} - {}", r.bot, r.top);
    }
}

fn main() {
    let contents =
        fs::read_to_string("src/15/input.txt").expect("Should have been able to read the file");
    part1(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_rock_lines() {
        let test_input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        assert_eq!(2 + 2, 4);
    }
}
