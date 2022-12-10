use grid::Grid;
use std::fs;

fn evaluate_registers(cycle_count: i32, register_value: i32, signal_strengths: &mut Vec<i32>) {
    if (cycle_count - 20) % 40 == 0 {
        signal_strengths.push(register_value * cycle_count);
    }
}

fn part1(contents: &str) {
    println!("Part 1");

    let mut register_value = 1;
    let mut cycle_count = 0;
    let mut signal_strengths: Vec<i32> = Vec::new();

    let lines = contents.split("\n");
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        match parts[0] {
            "noop" => {
                cycle_count += 1;
                evaluate_registers(cycle_count, register_value, &mut signal_strengths);
            }
            "" => continue,
            "addx" => {
                cycle_count += 1;
                evaluate_registers(cycle_count, register_value, &mut signal_strengths);
                cycle_count += 1;
                evaluate_registers(cycle_count, register_value, &mut signal_strengths);
                register_value += parts[1].parse::<i32>().unwrap();
            }
            _ => panic!("Unknown command {}", line),
        }
    }

    println!(
        "Sum of signal strengths: {}",
        signal_strengths.iter().sum::<i32>()
    );
}

fn draw_pixel(cycle_count: usize, register_value: i32) -> &'static str {
    let col = (cycle_count - 1) % 40;
    // println!(
    //     "cycle: {}, col: {}, register: {}",
    //     cycle_count, col, register_value
    // );

    let sprite_center = register_value % 40;
    let sprite_left = sprite_center - 1;
    let sprite_right = sprite_center + 1;

    if (col as i32) == sprite_center || (col as i32) == sprite_left || (col as i32) == sprite_right
    {
        return "#";
    } else {
        return ".";
    }
}

fn draw_screen(cycle_count: usize, register_value: i32, screen: &mut Grid<&str>) {
    let row = (cycle_count - 1) / 40;
    let col = (cycle_count - 1) % 40;

    assert!(row < screen.rows());
    assert!(col < screen.cols());

    screen[row][col] = draw_pixel(cycle_count, register_value);
}

fn print_screen(screen: &Grid<&str>) -> String {
    let row_count = screen.rows();
    let mut i = 0;

    let mut output: String = "".to_string();

    while i < row_count {
        if i != 0 {
            output.push_str("\n");
        }
        let row = screen
            .iter_row(i)
            .map(|k| format!("{}", k))
            .collect::<Vec<_>>()
            .join("");
        output.push_str(&row);
        i += 1;
    }
    output
}

fn run_program(contents: &str) -> String {
    let mut cycle_count = 0;
    let mut register_value = 1;
    let mut screen: Grid<&str> = Grid::new(6, 40);

    let lines = contents.split("\n");
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        match parts[0] {
            "noop" => {
                cycle_count += 1;
                draw_screen(cycle_count, register_value, &mut screen);
                // println!("{}\n\n", print_screen(&screen));
            }
            "" => continue,
            "addx" => {
                cycle_count += 1;
                draw_screen(cycle_count, register_value, &mut screen);
                // println!("{}\n\n", print_screen(&screen));
                cycle_count += 1;
                draw_screen(cycle_count, register_value, &mut screen);
                // println!("{}\n\n", print_screen(&screen));
                register_value += parts[1].parse::<i32>().unwrap();
            }
            _ => panic!("Unknown command {}", line),
        }
    }

    return print_screen(&screen);
}

fn part2(contents: &str) {
    println!("Part 2");

    println!("{}", run_program(contents));
}

fn main() {
    let contents =
        fs::read_to_string("src/10/input.txt").expect("Should have been able to read the file");
    part1(&contents);
    println!("");
    part2(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_draw_pixel() {
        assert_eq!(draw_pixel(1, 0), "#");
        assert_eq!(draw_pixel(3, 16), ".");
        assert_eq!(draw_pixel(9, 8), "#");
        assert_eq!(draw_pixel(10, 8), "#");
    }

    #[test]
    fn test_run_program() {
        let sample_program = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let expected_result = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_string();
        let result = run_program(sample_program);
        println!(
            "Result:\n{}\n\nExpected result:\n{}\n",
            result, expected_result
        );
        assert_eq!(result, expected_result);
    }
}
