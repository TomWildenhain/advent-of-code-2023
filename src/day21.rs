use std::fs;

type Grid<T> = Vec<Vec<T>>;

fn blank_grid<T>(width: usize, height: usize, item: T) -> Grid<T>
    where T: Copy {
    return (0..height).map(|_| (0..width).map(|_| item).collect()).collect();
}

type Direction = (isize, isize);

const DIRECTIONS: [Direction; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

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

fn neighbors(point: (usize, usize), width: usize, height: usize) -> impl Iterator<Item=(usize, usize)> {
    return DIRECTIONS.iter().filter_map(move |d| step_in_direction(point, *d, width, height));
}

fn step(grid: &Grid<u8>, locations: &Grid<bool>) -> Grid<bool> {
    let height = grid.len();
    let width = grid[0].len();

    let mut new_locations = blank_grid(width, height, false);

    for y in 0..height {
        for x in 0..width {
            if locations[y][x] {
                for (new_x, new_y) in neighbors((x, y), width, height) {
                    if grid[new_y][new_x] != b'#' {
                        new_locations[new_y][new_x] = true;
                    }
                }
            }
        }
    }

    return new_locations;
}

fn main() {
    let content = fs::read_to_string("./src/input21.txt").unwrap();
    let grid: Grid<_> = content.lines().map(|l| l.bytes().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut locations = blank_grid(width, height, false);

    let (start_x, start_y) = 
        grid.iter().enumerate().find_map(
            |(y, row)| row.iter().enumerate().find_map(
                |(x, c)| if *c == b'S' {Some((x, y))} else {None})).unwrap();

    locations[start_y][start_x] = true;

    for i in 0..64 {
        locations = step(&grid, &locations);
    }

    println!("{}", locations.iter().map(|row| row.iter().filter(|l| **l).count()).sum::<usize>());
}