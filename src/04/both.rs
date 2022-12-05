use range_collections::range_set::RangeSet2;
use range_collections::AbstractRangeSet;
use std::fs;

fn parse_line(line: &str) -> [(i32, i32); 2] {
    let assignment_pair = line.split(",");
    let mut ranges = [(0, 0), (0, 0)];
    let mut index = 0;
    for assignment in assignment_pair {
        let parts: Vec<i32> = assignment
            .split("-")
            .map(|part| part.parse::<i32>().unwrap())
            .collect();
        if parts.len() == 2 {
            ranges[index] = (parts[0], parts[1]);
        }
        index += 1;
    }
    ranges
}

fn part1() {
    println!("Part 1");

    let contents =
        fs::read_to_string("src/04/input.txt").expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut subsumed_count = 0;
    for line in lines {
        if line != "" {
            let ranges = parse_line(line);
            let range1: RangeSet2<i32> = RangeSet2::from((ranges[0].0)..(ranges[0].1 + 1)); // why doesn't ..= work here?
            let range2: RangeSet2<i32> = RangeSet2::from((ranges[1].0)..(ranges[1].1 + 1));
            let is_subset = range1.is_subset(&range2);
            let is_superset = range1.is_superset(&range2);
            if is_subset || is_superset {
                subsumed_count += 1;
            }
        }
    }
    println!("Total subsumed:\n{subsumed_count}");
}

fn part2() {
    println!("Part 2");

    let contents =
        fs::read_to_string("src/04/input.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n");
    let mut overlapping_count = 0;
    for line in lines {
        if line != "" {
            let ranges = parse_line(line);
            let range1: RangeSet2<i32> = RangeSet2::from((ranges[0].0)..(ranges[0].1 + 1)); // why doesn't ..= work here?
            let range2: RangeSet2<i32> = RangeSet2::from((ranges[1].0)..(ranges[1].1 + 1));
            if !range1.is_disjoint(&range2) {
                overlapping_count += 1;
            }
        }
    }
    println!("Total overlapping:\n{overlapping_count}");
}

fn main() {
    part1();
    part2();
}
