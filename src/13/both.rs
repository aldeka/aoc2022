use std::cmp::Ordering;
use std::fs;
use std::iter::zip;

#[derive(Eq, PartialEq, Debug)]
enum Packet {
    List(Box<Vec<Packet>>),
    Int(i64),
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Int(s) => {
                match other {
                    Packet::Int(o) => {
                        // compare as ints
                        return s.cmp(o);
                    }
                    Packet::List(_) => {
                        let self_as_list: Vec<Packet> = vec![Packet::Int(*s)];
                        return Packet::List(Box::new(self_as_list)).cmp(other);
                    }
                }
            }
            Packet::List(s) => {
                match other {
                    Packet::Int(o) => {
                        // I am an List, the other is a Int
                        let other_as_list = vec![Packet::Int(*o)];
                        return self.cmp(&Packet::List(Box::new(other_as_list)));
                    }
                    Packet::List(o) => {
                        // we're both lists
                        let iter = zip(s.iter(), o.iter());
                        for (s1, o1) in iter {
                            match s1.cmp(o1) {
                                Ordering::Greater => return Ordering::Greater,
                                Ordering::Less => return Ordering::Less,
                                _ => continue,
                            }
                        }
                        return s.len().cmp(&o.len());
                    }
                }
            }
        }
    }
}

fn parse_line(line: &str) -> Packet {
    let mut nesting: Vec<Vec<Packet>> = Vec::new();
    let mut curr: Vec<Packet> = Vec::new();
    let mut number_chars: Vec<char> = Vec::new();

    for char in line.chars() {
        match char {
            '[' => {
                nesting.push(curr);
                curr = Vec::new();
            }
            ']' => {
                if number_chars.len() > 0 {
                    let new_num = String::from_iter(number_chars).parse::<i64>().unwrap();
                    let packet = Packet::Int(new_num);
                    let boxed = packet;
                    curr.push(boxed);
                    number_chars = Vec::new();
                }

                let mut parent = nesting.pop().unwrap();
                parent.push(Packet::List(Box::new(curr)));
                curr = parent;
            }
            ',' => {
                if number_chars.len() > 0 {
                    let new_num = String::from_iter(number_chars).parse::<i64>().unwrap();
                    let packet = Packet::Int(new_num);
                    let boxed = packet;
                    curr.push(boxed);
                    number_chars = Vec::new();
                }
            }
            _ => {
                number_chars.push(char);
            }
        }
    }
    curr.pop().unwrap()
}

fn part1(contents: &str) {
    println!("Part 1");
    let pairs = contents.split("\n\n");
    let mut index = 1;
    let mut index_sum = 0;
    for pair in pairs {
        let mut pieces = pair.split("\n");
        let first = parse_line(pieces.next().unwrap().trim());
        let second = parse_line(pieces.next().unwrap().trim());

        if first < second {
            index_sum += index;
            // println!(
            //     "Well ordered: index {}:\n{:#?}\n{:#?}",
            //     index, first, second
            // );
        }

        index += 1;
    }

    println!("Index sum: {}", index_sum);
}

fn find_markers(contents: &str) -> usize {
    let mut lines = contents
        .split("\n")
        .filter(|line| line != &"\n" && line != &"")
        .map(|line| parse_line(line.trim()))
        .collect::<Vec<Packet>>();

    let divider_1: Packet =
        Packet::List(Box::new(vec![Packet::List(Box::new(vec![Packet::Int(
            2i64,
        )]))]));
    let divider_2: Packet =
        Packet::List(Box::new(vec![Packet::List(Box::new(vec![Packet::Int(
            6i64,
        )]))]));
    lines.push(divider_1);
    lines.push(divider_2);
    lines.sort();

    let index_1 = lines
        .iter()
        .position(|p| {
            p == &Packet::List(Box::new(vec![Packet::List(Box::new(vec![Packet::Int(
                2i64,
            )]))]))
        })
        .unwrap()
        + 1;
    let index_2 = lines
        .iter()
        .position(|p| {
            p == &Packet::List(Box::new(vec![Packet::List(Box::new(vec![Packet::Int(
                6i64,
            )]))]))
        })
        .unwrap()
        + 1;
    // println!("Sorted:\n{:#?}", lines);
    index_1 * index_2
}

fn part2(contents: &str) {
    println!("\nPart 2");
    println!("Answer: {}", find_markers(contents));
}

fn main() {
    let contents =
        fs::read_to_string("src/13/input.txt").expect("Should have been able to read the file");
    part1(&contents);
    part2(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let contents = "[1,1,3,1,1]
[1,1,5,1,1]";
        part1(&contents);
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_part2() {
        let contents = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(find_markers(contents), 140);
    }
}
