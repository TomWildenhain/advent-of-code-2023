use std::fs;

type Grid = Vec<Vec<u8>>;

fn blank_grid(width: usize, height: usize) -> Grid {
    return (0..height).map(|_| vec![b'.'; width]).collect();
}

fn tilt_grid_north(grid: &Grid) -> Grid {
    let height = grid.len();
    let width = grid[0].len();

    let mut new_grid: Grid = blank_grid(width, height);
    let mut open_indices: Vec<usize> = (0..width).map(|_| 0).collect();

    for y in 0..height {
        for x in 0..width {
            match grid[y][x] {
                b'.' => {

                },
                b'#' => {
                    new_grid[y][x] = b'#';
                    open_indices[x] = y + 1;
                },
                b'O' => {
                    new_grid[open_indices[x]][x] = b'O';
                    open_indices[x] += 1;
                }
                c => panic!("Invalid char {}", c)
            }
        }
    }

    return new_grid;
}

fn transpose(grid: &Grid) -> Grid {
    let height = grid.len();
    let width = grid[0].len();

    return (0..width).map(|x| (0..height).map(|y| grid[y][x]).collect()).collect();
}

fn flip_vertically(grid: &Grid) -> Grid {
    return grid.into_iter().rev().map(|row| row.to_owned()).collect();
}

fn spin_cycle(grid: &Grid) -> Grid {
    // North
    let grid: Grid = tilt_grid_north(&grid);
    
    // West
    let grid: Grid = transpose(&grid);
    let grid: Grid = tilt_grid_north(&grid);
    let grid: Grid = transpose(&grid);

    // South
    let grid: Grid = flip_vertically(&grid);
    let grid: Grid = tilt_grid_north(&grid);
    let grid: Grid = flip_vertically(&grid);

    // East
    let grid: Grid = transpose(&grid);
    let grid: Grid = flip_vertically(&grid);
    let grid: Grid = tilt_grid_north(&grid);
    let grid: Grid = flip_vertically(&grid);
    let grid: Grid = transpose(&grid);

    return grid;
}

fn get_load(grid: &Grid) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let mut sum = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'O' {
                sum += height - y
            }
        }
    }

    return sum;
}

fn part1() {
    let content = fs::read_to_string("./src/input14.txt").unwrap();
    let grid: Grid = content.lines().map(|l| l.bytes().collect()).collect();

    let new_grid = tilt_grid_north(&grid);

    println!("{}", get_load(&new_grid));
}

fn part2() {
    let content = fs::read_to_string("./src/input14.txt").unwrap();
    let grid: Grid = content.lines().map(|l| l.bytes().collect()).collect();

    let mut current_grid: Grid = grid;

    let mut cnt: usize = 0;

    // Period determined by reading print statements
    let cycle_length = 9;

    loop {
        current_grid = spin_cycle(&current_grid);
        cnt += 1;
        println!("Index: {}", cnt);
        println!("Load: {}", get_load(&current_grid));
        if cnt % cycle_length == 1000000000 % cycle_length && cnt > 200 {
            break;
        }
    }
}

fn main() {
    part2();
}