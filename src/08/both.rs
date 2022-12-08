use grid::*;
use std::fs;

#[derive(Debug)]
struct Tree {
    height: u32,
    is_seen: bool,
}

fn make_tree_grid(contents: &str) -> Grid<Tree> {
    let column_count = contents.split("\n").next().unwrap().len();

    let grid_contents = contents
        .chars()
        .filter(|char| char.to_string() != "\n".to_string())
        .map(|char| Tree {
            height: char.to_digit(10).unwrap(),
            is_seen: false,
        })
        .collect();

    let grid: Grid<Tree> = Grid::from_vec(grid_contents, column_count);
    // println!("Grid {:#?}", grid.size());
    grid
}

fn get_row_visible_tree_count<'a>(row: impl Iterator<Item = &'a mut Tree>) -> u32 {
    let mut count = 0;
    let mut tallest_tree: Option<u32> = None;
    for tree in row {
        if tallest_tree == None || tree.height > tallest_tree.unwrap() {
            tallest_tree = Some(tree.height);
            if !tree.is_seen {
                count += 1;
                tree.is_seen = true;
            }
        }
    }
    count
}

fn get_total_visible_tree_count(g: &mut Grid<Tree>) -> u32 {
    let mut visible_count = 0;
    // top and bottom
    let mut i = 0;
    while i < g.rows() {
        let row = g.iter_row_mut(i);
        visible_count += get_row_visible_tree_count(row);
        let rev_row = g.iter_row_mut(i).rev();
        visible_count += get_row_visible_tree_count(rev_row);
        i += 1;
    }
    // left and right;
    i = 0;
    while i < g.cols() {
        let col = g.iter_col_mut(i);
        visible_count += get_row_visible_tree_count(col);
        let rev_col = g.iter_col_mut(i).rev();
        visible_count += get_row_visible_tree_count(rev_col);
        i += 1;
    }
    visible_count
}

fn trees_from_point<'a>(row: Vec<&Tree>, start: usize) -> u32 {
    let mut count = 0;
    let start_height = row[start].height;
    let mut x = start + 1;

    while x < row.len() {
        count += 1;
        if row[x].height >= start_height {
            break;
        }
        x += 1;
    }

    count
}

fn part1(contents: &str) {
    println!("Part 1");
    let mut grid = make_tree_grid(contents);
    let visible_count = get_total_visible_tree_count(&mut grid);

    println!("Visible tree count:\n{}", visible_count);
}

fn part2(contents: &str) {
    println!("Part 2");
    let grid = make_tree_grid(contents);
    let mut max_scenic_score = 0;

    let mut i = 0;

    while i < grid.rows() {
        let mut j = 0;
        while j < grid.cols() {
            // let tree = grid.get(i, j).unwrap();
            // println!("({},{}): {}", i, j, tree.height);
            let north_count =
                trees_from_point(grid.iter_row(j).rev().collect(), grid.cols() - 1 - i);
            let south_count = trees_from_point(grid.iter_row(j).collect(), i);
            let west_count =
                trees_from_point(grid.iter_col(i).rev().collect(), grid.cols() - 1 - j);
            let east_count = trees_from_point(grid.iter_col(i).collect(), j);
            let score = north_count * south_count * west_count * east_count;
            if score > max_scenic_score {
                max_scenic_score = score;
            }
            j += 1;
        }
        i += 1;
    }
    println!("Max scenery:\n{}", max_scenic_score);
}

fn main() {
    let contents =
        fs::read_to_string("src/08/input.txt").expect("Should have been able to read the file");
    part1(&contents);
    println!("");
    part2(&contents);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_get_row_visible_tree_count() {
        let sample_input = "30373
25512
65332
33549
35390";
        let mut g = make_tree_grid(sample_input);
        let visible_tree_count = get_row_visible_tree_count(g.iter_row_mut(0));
        assert_eq!(visible_tree_count, 2);
        // only find newly-visible trees when we run it again
        let opposite_visible_tree_count = get_row_visible_tree_count(g.iter_row_mut(0).rev());
        assert_eq!(opposite_visible_tree_count, 1);
    }

    #[test]
    fn test_get_col_visible_tree_count() {
        let sample_input = "30373
25512
65332
33549
35390";
        let mut g = make_tree_grid(sample_input);
        println!("{:#?}", g);
        let mut col = g.iter_col_mut(1);
        let visible_tree_count = get_row_visible_tree_count(&mut col);
        println!("{:#?}", col);
        assert_eq!(visible_tree_count, 2);
        // only find newly-visible trees when we run it again
        let mut col_rev = g.iter_col_mut(1).rev();
        let opposite_visible_tree_count = get_row_visible_tree_count(&mut col_rev);
        println!("{:#?}", col_rev);
        assert_eq!(opposite_visible_tree_count, 1);
    }

    #[test]
    fn test_get_visible_tree_count() {
        let sample_input = "12345
12145
12345";
        let mut g = make_tree_grid(sample_input);
        let visible_tree_count = get_total_visible_tree_count(&mut g);
        println!("{:#?}", g);
        assert_eq!(visible_tree_count, 14);
    }
}
