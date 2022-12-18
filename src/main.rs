use std::fs;

enum Dim {
    X,
    Y,
    Z,
}

#[derive(Debug, Copy, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    faces: [bool; 6],
}

fn get_opposite_face(s: usize) -> usize {
    match s {
        0 => 5,
        1 => 4,
        2 => 3,
        3 => 2,
        4 => 1,
        5 => 0,
        _ => panic!("Invalid face {}", s),
    }
}

fn parse_line(line: &str) -> Cube {
    let coords = line
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    assert!(coords.len() == 3);
    Cube {
        x: coords[0],
        y: coords[1],
        z: coords[2],
        faces: [true, true, true, true, true, true],
    }
}

fn get_adjacent_air_cube(cubes: &mut Vec<Cube>, cube_index: usize, face: usize) -> Option<&Cube> {
    let testing_cube = cubes.get(cube_index).unwrap();
    let mut side_dim = Dim::X;
    let mut side_vec: i32 = 1; // either 1 or -1
    match face {
        0 => {}
        1 => side_dim = Dim::Y,
        2 => side_dim = Dim::Z,
        3 => {
            side_dim = Dim::Z;
            side_vec = -1;
        }
        4 => {
            side_dim = Dim::Y;
            side_vec = -1;
        }
        5 => side_vec = -1,
        _ => panic!("Invalid face {}", face),
    }

    let hypothetical_air_cube: Cube = match side_dim {
        Dim::X => Cube { x: testing_cube.x + side_vec, y: testing_cube.y, z: testing_cube.z, faces: [true, true, true, true, true, true]},
        Dim::Y => Cube { x: testing_cube.x, y: testing_cube.y + side_vec, z: testing_cube.z, faces: [true, true, true, true, true, true]},
        Dim::Z => Cube { x: testing_cube.x, y: testing_cube.y, z: testing_cube.z + side_vec, faces: [true, true, true, true, true, true]},
    };

    let adjacent_cube_index = cubes.iter().position(|c| hypothetical_air_cube.x == c.x && hypothetical_air_cube.y == c.y && hypothetical_air_cube.z == c.z );

    match adjacent_cube_index {
        Some(i) => {
            cubes[i].faces[get_opposite_face(face)] = false;
            cubes[cube_index].faces[face] = false;
            //return Some(&cubes[i]);
            return None
        }
        _ => {
            return Some(&hypothetical_air_cube);
    }
}

fn score_cube(cube: &Cube) -> usize {
    cube.faces
        .iter()
        .filter(|f| **f)
        .collect::<Vec<&bool>>()
        .len()
}

fn get_surface_area(cubes: &Vec<Cube>) -> usize {
    let mut count = 0;
    for cube in cubes.iter() {
        count += score_cube(cube);
    }
    count
}

fn part1(contents: &str) {
    println!("Part 1");

    let mut cubes: Vec<Cube> = Vec::new();

    for line in contents.trim().split("\n") {
        cubes.push(parse_line(line));
    }

    let mut i = 0;
    while i < cubes.len() {
        let mut j = 0;
        // check each face
        while j < 6 {
            // if it's (so far) uncovered
            if cubes[i].faces[j] {
                get_adjacent_air_cube(&mut cubes, i, j);
            }
            j += 1;
        }
        i += 1;
    }

    let answer = get_surface_area(&cubes);

    println!("Answer: {}", answer);
}

// fn part2(contents: &str) {
//     println!("Part 2");

//     let mut cubes: Vec<Cube> = Vec::new();

//     for line in contents.trim().split("\n") {
//         cubes.push(parse_line(line));
//     }

//     let mut air_cubes: Vec<Cube> = Vec::new();

//     let mut i = 0;
//     while i < cubes.len() {
//         let mut j = 0;
//         // check each face
//         while j < 6 {
//             // if it's (so far) uncovered
//             if cubes[i].faces[j] {
//                 match get_adjacent_air_cube(&mut cubes, i, j) {
//                     Some(a) => air_cubes.push(*a),
//                     None => {}
//                 }
//             }
//             j += 1;
//         }
//         i += 1;
//     }

//     // now we gotta check our air cubes to see if they're surrounded

// }

fn main() {
    let contents =
        fs::read_to_string("src/18/input.txt").expect("Should have been able to read the file");
    part1(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part_1() {
        assert_eq!(2 + 2, 4)
    }

    // #[test]
    // fn test_get_adjacent_cube() {
    //     let mut cubes = vec![parse_line("1,1,1"), parse_line("2,1,1")];
    //     let adj = get_adjacent_cube(&mut cubes, 0, 0);
    //     assert_eq!(adj.unwrap().x, 2);
    //     assert_eq!(adj.unwrap().faces, [true, true, true, true, true, false]);
    //     assert_eq!(
    //         cubes.get(0).unwrap().faces,
    //         [false, true, true, true, true, true]
    //     );
    // }
}
