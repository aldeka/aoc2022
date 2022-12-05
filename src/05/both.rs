use std::fs;

type BoatStack = Vec<char>;

type Boat = Vec<BoatStack>;

#[derive(Debug)]
struct Instruction {
    number_containers: u32,
    boat_from: usize,
    boat_to: usize,
}

fn make_boat(boat_lines: &mut Vec<&str>) -> Boat {
    let mut boat: Boat = Vec::new();
    let crate_size = 4;

    // pop off the column numbers, we don't need them
    boat_lines.pop();
    // reverse it so we stack things bottom-up
    boat_lines.reverse();

    for line in boat_lines.iter() {
        // a single layer of the stacks
        let mut index = 0;
        for char in line.chars() {
            if char != ' ' && char != '[' && char != ']' && char != '\n' {
                // we've found a container's letter contents
                // divide so we get which boat stack to stack this on
                let boat_index = (index - 1) / crate_size;
                if boat.len() == boat_index {
                    let new_boat_stack: BoatStack = Vec::new();
                    boat.push(new_boat_stack);
                }
                if boat.len() < boat_index {
                    panic!("Stack {boat_index} appears to be missing from the boat!");
                }
                boat[boat_index].push(char);
            }
            index += 1;
        }
    }
    boat
}

fn parse_instruction(line: &str) -> Instruction {
    let words: Vec<&str> = line.split(" ").collect();
    let inst = Instruction {
        number_containers: words[1].to_string().parse::<u32>().unwrap(),
        boat_from: words[3].to_string().parse::<usize>().unwrap() - 1,
        boat_to: words[5].to_string().parse::<usize>().unwrap() - 1,
    };
    inst
}

fn use_crane(boat: &mut Boat, instruction: Instruction) {
    let mut index = 0;
    while index < instruction.number_containers {
        let container = boat[instruction.boat_from].pop();
        match container {
            Some(char) => boat[instruction.boat_to].push(char),
            None => panic!("Crate to move did not exist"),
        }
        index += 1;
    }
}

fn use_new_crane(boat: &mut Boat, instruction: Instruction) {
    let mut moved_stack: Vec<char> = Vec::new();
    while moved_stack.len() < instruction.number_containers.try_into().unwrap() {
        let container = boat[instruction.boat_from].pop();
        match container {
            Some(char) => moved_stack.push(char),
            None => panic!("Crate to move didn't exist"),
        }
    }
    moved_stack.reverse();
    boat[instruction.boat_to].append(&mut moved_stack);
}

fn get_answer(boat: &mut Boat) {
    let mut answer: String = String::from("");
    for stack in boat {
        let letter = stack.pop().unwrap();
        answer.push(letter);
    }
    println!("Answer: {answer}");
}

fn part1() {
    println!("Part 1");

    let contents =
        fs::read_to_string("src/05/input.txt").expect("Should have been able to read the file");

    let lines = contents.split("\n");

    let mut boat_lines = Vec::new();
    let mut instruction_lines = Vec::new();

    let mut has_started_instructions = false;
    for line in lines {
        if line == "" {
            has_started_instructions = true;
        } else if has_started_instructions {
            instruction_lines.push(line);
        } else {
            boat_lines.push(line);
        }
    }
    let mut boat = make_boat(boat_lines);
    //println!("Boat:\n{boat:#?}");

    for line in instruction_lines {
        let inst = parse_instruction(line);
        // println!("{inst:#?}");
        use_crane(&mut boat, inst);
    }

    //println!("Boat:\n{boat:#?}");

    get_answer(&mut boat);
}

fn part2() {
    println!("Part 2");

    let contents =
        fs::read_to_string("src/05/input.txt").expect("Should have been able to read the file");
    let lines = contents.split("\n");

    let mut boat_lines = Vec::new();
    let mut instruction_lines = Vec::new();

    let mut has_started_instructions = false;
    for line in lines {
        if line == "" {
            has_started_instructions = true;
        } else if has_started_instructions {
            instruction_lines.push(line);
        } else {
            boat_lines.push(line);
        }
    }
    let mut boat = make_boat(boat_lines);

    for line in instruction_lines {
        let inst = parse_instruction(line);
        use_new_crane(&mut boat, inst);
    }

    get_answer(&mut boat);
}

fn main() {
    part1();
    part2();
}
