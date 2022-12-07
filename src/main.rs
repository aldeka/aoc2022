// use regex::Regex;
use std::fs;

struct File {
    name: String,
    size: u32,
}

struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl Dir {
    fn new<'a>(name: &'a str) -> Dir {
        Dir {
            name: name.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn add_file(&mut self, name: &str, size: u32) {
        self.files.push(File {
            name: name.to_string(),
            size: size,
        });
    }

    fn add_dir(&mut self, name: &str) {
        self.dirs.push(Dir {
            name: name.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
        });
    }
}

fn part1() {
    println!("Part 1");

    let contents =
        fs::read_to_string("src/07/input.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n");

    let mut location_history: Vec<&mut Dir> = Vec::new();
    let mut root = Dir::new("/");
    location_history.push(&mut root);

    for line in lines.into_iter() {
        let mut words = line.split(" ");
        let first_word = words.next().unwrap();
        if first_word == "$" {
            // it's a command
            let command = words.next().unwrap();
            if command == "cd" {
                let target = words.next().unwrap();
                println!("Change directory: {:#?}", target);
                match target {
                    "/" => {
                        location_history.clear();
                        location_history.push(&mut root);
                    }
                    ".." => {
                        location_history.pop();
                    }
                    _ => {
                        let current_dir = location_history.last().unwrap();
                        // are we going somewhere we've been before?
                        let new_dir = current_dir.dirs.iter_mut().find(|dir| dir.name == target);
                        match new_dir {
                            Some(x) => location_history.push(x),
                            None => {
                                // or do we need to make a new directory for where we're going
                                current_dir.dirs.push(Dir::new(target));
                                location_history.push(&mut current_dir.dirs.last().unwrap());
                            }
                        }
                    }
                }
            } else if command == "ls" {
                println!("List directory: {}", location_history.last().unwrap().name);
            } else {
                panic!("Unknown command:\n{command}");
            }
        } else {
            // we're getting back a listing of whatever the current location is
            // check items to see if they already exist and if not, add them
        }
    }
}

// fn part2() {
//     println!("Part 2");
// }

fn main() {
    part1();
    // part2();
}
