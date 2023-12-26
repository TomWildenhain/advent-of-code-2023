use std::cmp;
use std::fs;

type Grid<T> = Vec<Vec<T>>;

fn blank_grid<T>(width: usize, height: usize, item: T) -> Grid<T>
    where T: Copy {
    return (0..height).map(|_| (0..width).map(|_| item).collect()).collect();
}

// Bit order: DOWN, UP, RIGHT, LEFT
const DOWN: u8 = 1;
const UP: u8 = 2;
const RIGHT: u8 = 4;
const LEFT: u8 = 8;

type Direction = (isize, isize);

fn direction_to_mask(direction: Direction) -> u8 {
    match direction {
        (0, 1) => DOWN,
        (0, -1) => UP,
        (1, 0) => RIGHT,
        (-1, 0) => LEFT,
        _ => panic!("Invalid direction")
    }
}

fn step_in_direction(point: (usize, usize), direction: Direction, width: usize, height: usize) -> Option<(usize, usize)> {
    let (x, y) = point;
    let (dx, dy) = direction;

    // I put a question mark at the end!
    let new_x = x.checked_add_signed(dx)?;
    let new_y = y.checked_add_signed(dy)?;
    if new_x < width && new_y < height {
        return Some((new_x, new_y));
    }

    return None;
}

struct Context {
    width: usize,
    height: usize,
    grid: Grid<u8>,
    energy: Grid<u8>
}

fn fire_laser_forward(context: &mut Context, direction: Direction, point: (usize, usize)) {
    if let Some(new_point) = step_in_direction(point, direction, context.width, context.height) {
        fire_laser(context, direction, new_point);
    }
}

fn mark_visited(context: &mut Context, direction: Direction, point: (usize, usize)) -> bool {
    let mask = direction_to_mask(direction);
    let (x, y) = point;
    if context.energy[y][x] & mask == 0 {
        context.energy[y][x] |= mask;
        return true;
    }
    return false;
}

fn fire_laser(context: &mut Context, direction: Direction, point: (usize, usize)) {
    let (x, y) = point;
    let (dx, dy) = direction;
    let c = context.grid[y][x];
    if !mark_visited(context, direction, point) {
        return;
    }

    match c {
        b'\\' => {
            fire_laser_forward(context, (dy, dx), point);
        },
        b'/' => {
            fire_laser_forward(context, (-dy, -dx), point);
        },
        b'-' => {
            if dy != 0 {
                fire_laser_forward(context, (1, 0), point);
                fire_laser_forward(context, (-1, 0), point);
            } else {
                fire_laser_forward(context, direction, point);
            }
        },
        b'|' => {
            if dx != 0 {
                fire_laser_forward(context, (0, 1), point);
                fire_laser_forward(context, (0, -1), point);
            } else {
                fire_laser_forward(context, direction, point);
            }
        },
        b'.' => {
            fire_laser_forward(context, direction, point);
        }
        c => panic!("Invalid character {c}")
    }
}

fn energy_row_to_string(row: &Vec<u8>) -> String {
    let strs: Vec<_> = row.iter().map(|c| if *c != 0_u8 {"#"} else {"."}).collect();
    return strs.join("");
}

fn energy_grid_to_string(grid: &Grid<u8>) -> String {
    let strs: Vec<_> = grid.iter().map(energy_row_to_string).collect();
    return strs.join("\n");
}

fn count_energized(grid: &Grid<u8>) -> usize {
    return grid.iter().map(|row| row.iter().filter(|c| **c != 0).count()).sum();
}

fn fire_laser_and_count(grid: &Grid<u8>, start_direction: Direction, start_point: (usize, usize)) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let energy: Grid<u8> = blank_grid(width, height, 0);

    let mut context = Context {
        width: width,
        height: height,
        grid: grid.clone(),
        energy: energy,
    };

    fire_laser(&mut context, start_direction, start_point);

    return count_energized(&context.energy);
}

fn part2() {
    let content = fs::read_to_string("./src/input16.txt").unwrap();
    let grid: Grid<_> = content.lines().map(|l| l.bytes().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut max_energized = 0;
    for y in 0..height {
        max_energized = cmp::max(max_energized, fire_laser_and_count(&grid, (1, 0), (0, y)));
        max_energized = cmp::max(max_energized, fire_laser_and_count(&grid, (-1, 0), (width-1, y)));
    }
    for x in 0..width {
        max_energized = cmp::max(max_energized, fire_laser_and_count(&grid, (0, 1), (x, 0)));
        max_energized = cmp::max(max_energized, fire_laser_and_count(&grid, (0, -1), (x, height-1)));
    }

    println!("{max_energized}");
}

fn part1() {
    let content = fs::read_to_string("./src/input16.txt").unwrap();
    let grid: Grid<_> = content.lines().map(|l| l.bytes().collect()).collect();

    println!("{}", fire_laser_and_count(&grid, (1, 0), (0, 0)));
}

use std::thread;

const STACK_SIZE: usize = 4 * 1024 * 1024;

fn main() {
    // Spawn thread with explicit stack size (https://www.reddit.com/r/rust/comments/872fc4/how_to_increase_the_stack_size/)
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(part2)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}