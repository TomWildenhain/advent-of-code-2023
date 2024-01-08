use std::fs;

type Grid<T> = Vec<Vec<T>>;

fn blank_grid<T: Copy>(width: usize, height: usize, item: T) -> Grid<T> {
    return (0..height).map(|_| (0..width).map(|_| item).collect()).collect();
}

type Direction = (isize, isize);

const DIRECTIONS: [Direction; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn step_in_direction(point: (usize, usize), direction: Direction, width: usize, height: usize) -> Option<(usize, usize)> {
    let (x, y) = point;
    let (dx, dy) = direction;

    let new_x = x.checked_add_signed(dx)?;
    let new_y = y.checked_add_signed(dy)?;
    if new_x < width && new_y < height {
        return Some((new_x, new_y));
    }

    return None;
}

fn neighbors(point: (usize, usize), width: usize, height: usize) -> impl Iterator<Item=(usize, usize)> {
    return DIRECTIONS.iter().filter_map(move |d| step_in_direction(point, *d, width, height));
}

fn char_to_direction(char: u8) -> Option<Direction> {
    match char {
        b'>' => Some((1, 0)),
        b'<' => Some((-1, 0)),
        b'v' => Some((0, 1)),
        b'^' => Some((0, -1)),
        _ => None
    }
}

fn row_to_string(row: &Vec<bool>) -> String {
    let strs: Vec<_> = row.iter().map(|c| if *c {"#"} else {"."}).collect();
    return strs.join("");
}

fn grid_to_string(grid: &Grid<bool>) -> String {
    let strs: Vec<_> = grid.iter().map(row_to_string).collect();
    return strs.join("\n");
}

struct Context {
    height: usize,
    width: usize,
    grid: Grid<u8>,
    visited: Grid<bool>,
    end: (usize, usize),
    best_yet: usize,
}

fn longest_path_from_point_impl(context: &mut Context, current_length: usize, location: (usize, usize)) -> usize {
    let (x, y) = location;

    if location == context.end {
        if current_length > context.best_yet {
            println!("{}", current_length);
            context.best_yet = current_length;
        }
        return current_length;
    }

    if let Some((dx, dy)) = char_to_direction(context.grid[y][x]) {
        let (new_x, new_y) = (x.saturating_add_signed(dx), y.saturating_add_signed(dy));
        if context.visited[new_y][new_x] {
            return current_length;
        }
        else {
            return longest_path_from_point(context, current_length + 1, (new_x, new_y));
        }
    }

    let mut longest_length = 0;
    for (new_x, new_y) in neighbors(location, context.width, context.height) {
        if context.grid[new_y][new_x] != b'#' && !context.visited[new_y][new_x] {
            longest_length = longest_length.max(longest_path_from_point(context, current_length + 1, (new_x, new_y)));
        }
    }

    return longest_length;
}

fn longest_path_from_point(context: &mut Context, current_length: usize, location: (usize, usize)) -> usize {
    let (x, y) = location;
    context.visited[y][x] = true;
    let res = longest_path_from_point_impl(context, current_length, location);
    context.visited[y][x] = false;
    return res;
}

fn part1() {
    let content = fs::read_to_string("./src/input23.txt").unwrap();
    let grid: Grid<u8> = content.lines().map(|l| l.bytes().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut visited = blank_grid(width, height, false);

    let start_x = content.lines().next().unwrap().find(".").unwrap();
    let start_y = 0;

    let end_x = content.lines().last().unwrap().find(".").unwrap();
    let end_y = height - 1;

    assert_eq!(grid[start_y][start_x], b'.');

    let mut context = Context {
        grid: grid, 
        visited: visited, 
        end: (end_x, end_y),
        height: height,
        width: width,
        best_yet: 0
    };

    println!("{}", longest_path_from_point(&mut context, 0, (start_x, start_y)));

    // 6474 is too low
}

use std::thread;

const STACK_SIZE: usize = 8 * 1024 * 1024;

fn main() {
    // Spawn thread with explicit stack size (https://www.reddit.com/r/rust/comments/872fc4/how_to_increase_the_stack_size/)
    let child = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(part1)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}