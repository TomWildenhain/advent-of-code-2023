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

fn count_true(grid: &Grid<bool>) -> usize {
    return grid.iter().map(|row| row.iter().filter(|l| **l).count()).sum::<usize>();
}

fn count_reachable_after_steps(grid: &Grid<u8>, start: (usize, usize), steps: usize) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let (start_x, start_y) = start;

    let mut locations = blank_grid(width, height, false);
    locations[start_y][start_x] = true;

    for i in 0..steps {
        locations = step(&grid, &locations);
    }

    return count_true(&locations);
}

fn part1() {
    let content = fs::read_to_string("./src/input21.txt").unwrap();
    let grid: Grid<_> = content.lines().map(|l| l.bytes().collect()).collect();

    let (start_x, start_y) = 
        grid.iter().enumerate().find_map(
            |(y, row)| row.iter().enumerate().find_map(
                |(x, c)| if *c == b'S' {Some((x, y))} else {None})).unwrap();

    println!("{}", count_reachable_after_steps(&grid, (start_x, start_y), 64));
}

#[derive(Debug)]
struct StartsAndSteps {
    start: (usize, usize),
    steps: usize,
    count: usize,
}

fn compute_number_completed_grids(grid_size: usize, total_steps: usize) -> (usize, usize) {
    let n = (total_steps + 1) / grid_size - 1;

    let total_completed = 4 * (n * (n + 1) / 2) + 1;

    let red_n = (n - 1) / 2;
    let black_n = n / 2;

    let completed_black = 4 * (black_n * (black_n + 1)) + 1;
    let completed_red = 4 * (red_n + 1) * (red_n + 1);

    assert_eq!(completed_black + completed_red, total_completed);

    return (completed_black, completed_red);
}

fn get_starts_and_steps(grid_size: usize, total_steps: usize) -> Vec<StartsAndSteps> {
    let mut result: Vec<StartsAndSteps> = vec![];

    let half_size = grid_size / 2;

    println!("{}", half_size * half_size);

    let steps_to_complete_black = half_size * 2 + (total_steps % 2);
    let steps_to_complete_red = half_size * 2 + ((total_steps + 1) % 2);

    // Add entries for completed grids.
    let (completed_black, completed_red) = compute_number_completed_grids(grid_size, total_steps);
    result.push(StartsAndSteps {start: (half_size, half_size), steps: steps_to_complete_black, count: completed_black});
    result.push(StartsAndSteps {start: (half_size, half_size), steps: steps_to_complete_red, count: completed_red});

    let steps_for_rhombus_corners = (total_steps - half_size - 1) % grid_size;
    println!("steps_for_rhombus_corners={}", steps_for_rhombus_corners);
    result.push(StartsAndSteps {start: (0, half_size), steps: steps_for_rhombus_corners, count: 1});
    result.push(StartsAndSteps {start: (grid_size - 1, half_size), steps: steps_for_rhombus_corners, count: 1});
    result.push(StartsAndSteps {start: (half_size, 0), steps: steps_for_rhombus_corners, count: 1});
    result.push(StartsAndSteps {start: (half_size, grid_size - 1), steps: steps_for_rhombus_corners, count: 1});

    // A second set of rhomus corners may be further along but not completed.
    let steps_for_old_rhombus_corners = steps_for_rhombus_corners + grid_size;
    let steps_to_complete_rhombus_corners = 3 * half_size;
    if steps_for_old_rhombus_corners < steps_to_complete_rhombus_corners {
        result.push(StartsAndSteps {start: (0, half_size), steps: steps_for_old_rhombus_corners, count: 1});
        result.push(StartsAndSteps {start: (grid_size - 1, half_size), steps: steps_for_old_rhombus_corners, count: 1});
        result.push(StartsAndSteps {start: (half_size, 0), steps: steps_for_old_rhombus_corners, count: 1});
        result.push(StartsAndSteps {start: (half_size, grid_size - 1), steps: steps_for_old_rhombus_corners, count: 1});
    }

    let rhombus_sides_cnt = (total_steps - 1) / grid_size;
    let steps_for_rhombus_sides = (total_steps - grid_size - 1) % grid_size;
    result.push(StartsAndSteps {start: (0, 0), steps: steps_for_rhombus_sides, count: rhombus_sides_cnt});
    result.push(StartsAndSteps {start: (0, grid_size - 1), steps: steps_for_rhombus_sides, count: rhombus_sides_cnt});
    result.push(StartsAndSteps {start: (grid_size - 1, 0), steps: steps_for_rhombus_sides, count: rhombus_sides_cnt});
    result.push(StartsAndSteps {start: (grid_size - 1, grid_size - 1), steps: steps_for_rhombus_sides, count: rhombus_sides_cnt});

    let old_rhombus_sides_cnt = rhombus_sides_cnt - 1;
    let steps_for_old_rhombus_sides = steps_for_rhombus_sides + grid_size;
    let steps_to_complete_rhombus_sides = 4 * half_size;
    if steps_for_old_rhombus_sides < steps_to_complete_rhombus_sides {
        result.push(StartsAndSteps {start: (0, 0), steps: steps_for_old_rhombus_sides, count: old_rhombus_sides_cnt});
        result.push(StartsAndSteps {start: (0, grid_size - 1), steps: steps_for_old_rhombus_sides, count: old_rhombus_sides_cnt});
        result.push(StartsAndSteps {start: (grid_size - 1, 0), steps: steps_for_old_rhombus_sides, count: old_rhombus_sides_cnt});
        result.push(StartsAndSteps {start: (grid_size - 1, grid_size - 1), steps: steps_for_old_rhombus_sides, count: old_rhombus_sides_cnt});
    }

    return result;
}

fn part2() {
    let content = fs::read_to_string("./src/input21.txt").unwrap();
    let grid: Grid<_> = content.lines().map(|l| l.bytes().collect()).collect();

    let size = grid.len();
    assert_eq!(size, grid[0].len(), "Grid must be square");
    
    let (start_x, start_y) = 
    grid.iter().enumerate().find_map(
        |(y, row)| row.iter().enumerate().find_map(
            |(x, c)| if *c == b'S' {Some((x, y))} else {None})).unwrap();

    assert_eq!((start_x, start_y), (size / 2, size / 2), "Must start at center");

    let mut sum = 0;
    for entry in get_starts_and_steps(size, 26501365) {
        sum += count_reachable_after_steps(&grid, entry.start, entry.steps) * entry.count;
    }

    println!("{}", sum);
}

fn main() {
    part2();
}