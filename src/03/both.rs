use core::str::Chars;
use std::collections::HashSet;
use std::fs;

fn get_char_value(c: char) -> u32 {
    let mut c_value = u32::from(c);
    if c_value > 96 {
        // lowercase
        c_value -= 96;
    } else {
        // uppercase
        c_value -= 38;
    }
    // println!("{c}: {c_value}");
    c_value
}

fn find_duplicate(a: Chars, b: Chars) -> char {
    let mut items_a: HashSet<char> = HashSet::new();
    for ai in a {
        items_a.insert(ai);
    }
    for bi in b {
        if items_a.contains(&bi) {
            return bi;
        }
    }
    panic!("Did not find overlapping item");
}

fn find_triplicate(a: Chars, b: Chars, c: Chars) -> char {
    let mut items_a: HashSet<char> = HashSet::new();
    let mut a_b_overlap: HashSet<char> = HashSet::new();
    for ai in a {
        items_a.insert(ai);
    }
    for bi in b {
        if items_a.contains(&bi) {
            // the known set of overlaps between b and a
            a_b_overlap.insert(bi);
        }
    }
    for ci in c {
        // if it's present in b and a and matches with c...
        if a_b_overlap.contains(&ci) {
            return ci;
        }
    }
    panic!("Did not find overlapping item!");
}

fn part1() {
    println!("Part 1");

    let contents =
        fs::read_to_string("src/03/input.txt").expect("Should have been able to read the file");

    let lines = contents.split("\n");
    let mut total_misplaced_value = 0;
    for line in lines {
        if line != "" {
            let length = line.chars().count();
            let bag1 = &line[..length / 2];
            let bag2 = &line[length / 2..];
            let dup = find_duplicate(bag1.chars(), bag2.chars());
            total_misplaced_value += get_char_value(dup);
        }
    }
    println!("Final score:\n{total_misplaced_value}");
}

fn part2() {
    println!("Part 2");
    let contents =
        fs::read_to_string("src/03/input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents
        .split("\n")
        .filter(|line| match &line[..] {
            "" => return false,
            _ => return true,
        })
        .collect();
    let data_length = lines.len();
    if data_length % 3 != 0 {
        panic!("Should have threesomes of lines: {data_length}");
    }

    let mut total_badges_value = 0;

    let mut set_of_bags: Vec<&str> = Vec::new();

    for line in lines {
        if set_of_bags.len() == 2 {
            set_of_bags.push(line);

            let badge = find_triplicate(
                set_of_bags[0].chars(),
                set_of_bags[1].chars(),
                set_of_bags[2].chars(),
            );
            total_badges_value += get_char_value(badge);

            // reset for next threesome of bags
            set_of_bags = Vec::new();
        } else {
            set_of_bags.push(line);
        }
    }

    println!("Final score:\n{total_badges_value}");
}

fn main() {
    part1();
    part2();
}
