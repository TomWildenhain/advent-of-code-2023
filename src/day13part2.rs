use std::fs;
use std::cmp::min;

type Grid = Vec<Vec<u8>>;

fn has_mirror_at(grid: &Grid, y: usize) -> bool {
    if y == grid.len() - 1 {
        return false;
    }
    let dy_max = min(y, grid.len() - y - 2);
    for dy in 0..=dy_max {
        let y1 = y - dy;
        let y2 = y + 1 + dy;

        if grid[y1] != grid[y2] {
            return false;
        }
    }
    return true;
}

fn find_mirror(mut grid: Grid) -> Option<usize> {
    let height = grid.len();
    let width = grid[0].len();
    for x_flip in 0..width {
        for y_flip in 0..height {
            for y in 0..grid.len() {
                if !has_mirror_at(&grid, y) {
                    grid[y_flip][x_flip] = flip(grid[y_flip][x_flip]);
                    if has_mirror_at(&grid, y) {
                        return Some(y);
                    }
                    grid[y_flip][x_flip] = flip(grid[y_flip][x_flip]);
                }
            }
        }
    }

    return None;
}

fn transpose(grid: &Grid) -> Grid {
    let height = grid.len();
    let width = grid[0].len();

    return (0..width).map(|x| (0..height).map(|y| grid[y][x]).collect()).collect();
}

fn find_line_of_reflection_summary_value(grid: Grid) -> Option<usize> {
    let grid_transpose = transpose(&grid);
    let mirror_row = find_mirror(grid);
    let mirror_col = find_mirror(grid_transpose);
    if let Some(row) = mirror_row {
        return Some((row + 1) * 100);
    } else if let Some(col) = mirror_col {
        return Some(col + 1);
    }
    return None
}

fn flip(entry: u8) -> u8 {
    return match entry {
        b'.' => b'#',
        b'#' => b'.',
        _ => panic!("Unexpected character {}", entry)
    }
}

fn main() {
    let content = fs::read_to_string("./src/input13.txt").unwrap();
    let grids: Vec<Vec<Vec<u8>>> = content.split("\n\n").map(|s| s.lines().map(|l| l.as_bytes().to_vec()).collect()).collect();

    let mut sum = 0;
    for grid in grids {
        sum += find_line_of_reflection_summary_value(grid).unwrap();
    }

    println!("{}", sum);
}