use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Clone)]
struct Valve {
    name: String,
    connection_names: Vec<String>,
    flow: usize,
    is_open: bool,
}

fn parse_line(line: &str) -> Valve {
    let name = String::from(&line.to_string()[6..8]);
    let flow_string = String::from(line.to_string().split(";").nth(0).unwrap());
    let flow = flow_string[23..].parse::<usize>().unwrap();
    let connection_names: Vec<String> = line.to_string()[49..]
        .split(", ")
        .map(|n| n.to_string())
        .collect::<Vec<String>>();

    println!("Valve: {}, {}, {:#?}", name, flow, connection_names);
    Valve {
        name,
        flow,
        connection_names,
        is_open: false,
    }
}

fn make_valve_map(input: &str) -> HashMap<String, Valve> {
    let mut tunnels: HashMap<String, Valve> = HashMap::new();

    for line in input.trim().split("\n") {
        let tunnel = parse_line(line.trim());
        tunnels.insert(tunnel.name.clone(), tunnel);
    }
    tunnels
}

fn calculate_current_flow(map: &HashMap<String, Valve>) -> usize {
    let mut flow: usize = 0;
    for (_, v) in map.iter() {
        if v.is_open {
            flow += v.flow;
        }
    }
    flow
}

fn is_all_open(map: &HashMap<String, Valve>) -> bool {
    for (_, v) in map.iter() {
        if !v.is_open {
            return false;
        }
    }
    true
}

const MAX_STEP_COUNT: usize = 30;

fn pathfinder(
    map: &mut HashMap<String, Valve>,
    path: &mut Vec<String>,
    step: usize,
    acc_flow: usize,
    max_acc_flow: usize,
) -> usize {
    if is_all_open(map) || step == MAX_STEP_COUNT {
        // no movement matters anymore, let's just add up the remaining flow
        return max_acc_flow + (MAX_STEP_COUNT - calculate_current_flow(map));
    }
    if step > MAX_STEP_COUNT {
        panic!("Path got too long!, {:#?}", path);
    }

    match map.get_mut(&path[path.len() - 1]) {
        Some(v) => {
            // calculate this step's flow
            let new_acc_flow = calculate_current_flow(map) + acc_flow;
            let new_max_acc_flow = usize::max(max_acc_flow, new_acc_flow);

            if !v.is_open {
                v.is_open = true;
                // open it if it's not already
                return pathfinder(map, path, step + 1, new_acc_flow, new_max_acc_flow);
            } else {
                // this isn't a "open valve" step
                return v
                    .connection_names
                    .iter()
                    .map(|c| {
                        // run it again on each possible next step
                        let mut new_path: Vec<String> = path.clone();
                        new_path.push(c.to_string());
                        return pathfinder(
                            map,
                            &mut new_path,
                            step + 1,
                            new_acc_flow,
                            new_max_acc_flow,
                        );
                    })
                    .max()
                    .unwrap();
            }
        }
        None => return max_acc_flow,
    }
}

fn part1(contents: &str) -> usize {
    println!("Part 1");

    let mut valve_map = make_valve_map(contents);

    let max_flow = pathfinder(&mut valve_map, &mut vec!["AA".to_string()], 1, 0, 0);
    println!("Max flow: {}", max_flow);
    max_flow
}

fn main() {
    let contents =
        fs::read_to_string("src/16/input.txt").expect("Should have been able to read the file");
    part1(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse_line() {
        let line = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        assert_eq!(
            parse_line(line),
            Valve {
                name: String::from("AA"),
                connection_names: vec![String::from("DD"), String::from("II"), String::from("BB")],
                flow: 0,
                is_open: false,
            }
        )
    }

    #[test]
    fn test_valve_map() {
        let test_input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        let map = make_valve_map(test_input);
        assert_eq!(map.len(), 10);
        assert_eq!(
            map.get("EE").unwrap(),
            &parse_line("Valve EE has flow rate=3; tunnels lead to valves FF, DD")
        );
    }

    #[test]
    fn test_part1() {
        let test_input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(part1(test_input), 1651);
    }
}
