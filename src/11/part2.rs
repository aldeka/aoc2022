use std::{fs, collections::VecDeque, num::ParseIntError};

struct Monkey {
    name: usize,
    items: VecDeque<usize>,
    on_inspect: Box<dyn Fn(usize) -> usize>,
    test_divisor: usize,
    throw_to_true: usize,
    throw_to_false: usize,
    inspect_count: usize,
    lcm: Option<usize>,
}

impl Monkey {
    fn new(name: usize,
        items: VecDeque<usize>,
        on_inspect: Box<dyn Fn(usize) -> usize>,
        test_divisor: usize,
        throw_to_true: usize,
        throw_to_false: usize,
        lcm: Option<usize>
    ) -> Monkey
    {
        Monkey {
            name,
            items,
            on_inspect,
            test_divisor,
            throw_to_true,
            throw_to_false,
            inspect_count: 0,
            lcm
        }
    }

    fn set_lcm(&mut self, lcm: usize) {
        self.lcm = Some(lcm);
    }

    fn inspect(&mut self) {
        self.inspect_count += self.items.len();
        Some(self.lcm).expect("LCM not found!");
        self.items = self.items.iter().map(|item| (self.on_inspect)(*item) % self.lcm.unwrap()).collect();
    }

    fn throw(&mut self) -> usize {
        self.items.pop_front().unwrap()
    }

    fn get_destination_monkey(&self, item: usize) -> usize {
        if item % self.test_divisor == 0 {
            return self.throw_to_true;
        }
        self.throw_to_false
    }

    fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

// Monkey 0:
//   Starting items: 98, 97, 98, 55, 56, 72
//   Operation: new = old * 13
//   Test: divisible by 11
//     If true: throw to monkey 4
//     If false: throw to monkey 7


fn make_operation(operation_line: &str) -> Box<dyn Fn(usize) -> usize> {
    if operation_line.find("+") != None {
        // addition, which always has a static side to it
        let static_val = operation_line.split(" ")
            .find(|operand| {
                match operand.parse::<usize>() {
                    Ok(x) => return true,
                    Err(_) => return false,
                }
            })
            .unwrap().parse::<usize>().unwrap();
        return Box::new(move |b: usize| -> usize {
            static_val + b
        });
    } else {
        // it's a multiplication one
        let static_val_str = operation_line.split(" ")
        .find(|operand| {
            match operand.parse::<usize>() {
                Ok(x) => return true,
                Err(_) => return false,
            }
        });
        if static_val_str != None {
            let static_val = static_val_str.unwrap().parse::<usize>().unwrap();
            return Box::new(move |b: usize| -> usize {
                b * static_val
            });
        } else {
            return Box::new(move |b: usize| -> usize {
                b * b
            });
        }
    }
}

fn parse_monkeys(contents: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut primes: Vec<usize> = Vec::new();
    for monkey_spec in contents.split("\n\n") {
        let lines: Vec<&str> = monkey_spec.split("\n")
            .filter(|line| line != &"")
            .collect::<Vec<&str>>();
        let name: usize = lines[0].trim().split(" ")
            .filter(|line| {
                return line != &"" && line != &"Monkey";
            })
            .map(|line| line.replace(":", ""))
            .collect::<Vec<String>>()[0]
            .parse::<usize>().unwrap();
        let starting_items: VecDeque<usize> = lines[1].trim().split("Starting items: ")
            .collect::<VecDeque<&str>>()[1]
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap()).collect();
        println!("Monkey {name}: {starting_items:#?}");
        println!("***");

        let operation = make_operation(
            lines[2].replace("Operation: ", "")
            .as_str()
        );
        let test_divisor: usize = lines[3]["  Test: divisible by ".len()..].parse::<usize>().unwrap();
        primes.push(test_divisor);
        let throw_to_true: usize = lines[4]["    If true: throw to monkey ".len()..].parse::<usize>().unwrap();
        let throw_to_false: usize = lines[5]["    If false: throw to monkey ".len()..].parse::<usize>().unwrap();

        let lcm = None::<usize>;
        let monkey = Monkey::new(name, starting_items, operation, test_divisor, throw_to_true, throw_to_false, lcm);
        monkeys.push(monkey);
    }
    let lcm = primes.iter().product();
    for i in 0..monkeys.len() {
        monkeys[i].set_lcm(lcm);
    }
    monkeys
}

fn part2(contents: &str) {
    println!("Part 2");
    let mut monkeys = parse_monkeys(contents);
    for round in 1..10001 {
        println!("\nRound {}:", round);
        for i in 0..monkeys.len() {
            monkeys[i].inspect();
            while monkeys[i].items.len() > 0 {
                let thrown_item = monkeys[i].throw();
                let dest = monkeys[i].get_destination_monkey(thrown_item);
                monkeys[dest].catch(thrown_item);
            }
        }
        for i in 0..monkeys.len() {
            println!("Monkey {}: {:#?}", monkeys[i].name, monkeys[i].items);
        }
    }
    let mut inspect_counts = monkeys.iter().map(|monk| monk.inspect_count).collect::<Vec<usize>>();
    inspect_counts.sort();
    println!("Answer: {}", inspect_counts.pop().unwrap() * inspect_counts.pop().unwrap());
}

fn main() {
    let contents =
        fs::read_to_string("src/11/input.txt").expect("Should have been able to read the file");
    part2(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    // #[test]
    // fn test_part1() {
    //     let contents =
    //     fs::read_to_string("src/11/test_input.txt").expect("Should have been able to read the file");
    //     part1(&contents);
    //     assert_eq!(2+2, 4);
    // }
}
