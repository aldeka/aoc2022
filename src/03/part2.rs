use core::str;
use std::collections::HashSet;
use std::fs;

fn find_triplicate(a: str::Chars, b: str::Chars, c: str::Chars) -> char {
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

fn main() {
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

    // let expected_count = data_length / 3;
    // let mut line_count = 0;
    // let mut set_count = 0;
    let mut set_of_bags: Vec<&str> = Vec::new();

    for line in lines {
        // line_count += 1;
        if set_of_bags.len() == 2 {
            set_of_bags.push(line);
            // set_count += 1;
            // println!("bags {set_count} of {expected_count}, line {line_count}:\n{set_of_bags:#?}");

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
