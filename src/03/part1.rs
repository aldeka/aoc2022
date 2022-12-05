use core::str;
use std::collections::HashSet;
use std::fs;

fn find_duplicate(a: str::Chars, b: str::Chars) -> char {
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

fn get_char_value(c: char) -> u32 {
    let mut c_ascii = u32::from(c);
    if c_ascii > 96 {
        // lowercase
        c_ascii -= 96;
    } else {
        // uppercase
        c_ascii -= 38;
    }
    println!("{c}: {c_ascii}");
    c_ascii
}

fn main() {
    println!("Hello");

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
    // let test = get_char_value("A".chars().next().expect("No char found"));
    // println!("Test: {test}");
    println!("Final score:\n{total_misplaced_value}");
}
