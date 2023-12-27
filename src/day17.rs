use std::cmp;
use std::fs;

type Grid<T> = Vec<Vec<T>>;

fn blank_grid<T>(width: usize, height: usize, item: T) -> Grid<T>
    where T: Copy {
    return (0..height).map(|_| (0..width).map(|_| item).collect()).collect();
}

struct Context {
    width: usize,
    height: usize,
    grid: Grid<i32>,
    costs: Vec<Vec<Grid<i32>>>,
    min_blocks_before_turning: usize,
    max_blocks_without_turning: usize,
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

fn is_opposite(d1: Direction, d2: Direction) -> bool {
    let (x1, y1) = d1;
    let (x2, y2) = d2;

    return x1 == -x2 && y1 == -y2;
}

fn decrease_costs(context: &mut Context) -> bool {
    let mut progress = false;

    for (d_i, d1) in DIRECTIONS.iter().enumerate() {
        for r in 1..=context.max_blocks_without_turning {
            for (d2_i, d2) in DIRECTIONS.iter().enumerate() {
                let r2 = if d_i == d2_i {r + 1} else {1};
                if r < context.min_blocks_before_turning && d_i != d2_i {
                    continue;
                }

                if r2 > context.max_blocks_without_turning || is_opposite(*d1, *d2) {
                    continue;
                }

                for y in 0..context.height {
                    for x in 0..context.width {
                        if let Some((x2, y2)) = step_in_direction((x, y), *d2, context.width, context.height) {
                            let new_val = context.grid[y2][x2] + context.costs[d2_i][r2][y2][x2];
                            if new_val < context.costs[d_i][r][y][x] {
                                context.costs[d_i][r][y][x] = new_val;
                                progress = true;
                            }
                        }
                    }
                }
            }
        }
    }

    return progress;
}

fn print_costs(costs: &Grid<i32>) {
    for row in costs {
        println!("{:?}", row);
    }
}

fn part_1_and_2(min_blocks_before_turning: usize, max_blocks_without_turning: usize) {
    let content = fs::read_to_string("./src/input17.txt").unwrap();
    let grid: Grid<i32> = content.lines().map(
        |l| l.chars().map(
            |c| c.to_digit(10).unwrap().try_into().unwrap()
        ).collect()
    ).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut costs: Vec<Vec<Grid<i32>>> = DIRECTIONS.iter().map(
        |_| (0..=max_blocks_without_turning).map(
            |i| if i == 0 {vec![]} else {blank_grid(width, height, i32::MAX / 4)}).collect()).collect();

    for grids_for_dir in costs.iter_mut() {
        for cost_grid in grids_for_dir.iter_mut() {
            if cost_grid.len() > 0 {
                cost_grid[height - 1][width - 1] = 0;
            }
        }
    }
    
    let mut context = Context {
        height: height,
        width: width,
        grid: grid,
        costs: costs,
        min_blocks_before_turning: min_blocks_before_turning,
        max_blocks_without_turning: max_blocks_without_turning
    };

    while decrease_costs(&mut context) {
        // Decreasing costs...
    }

    let min_cost = context.costs.iter().map(|grids_for_dir| grids_for_dir[1][0][0]).min().unwrap();

    // print_costs(&context.costs[0][1]);
    println!("{min_cost}");
}

fn main() {
    //part_1_and_2(1, 3);
    part_1_and_2(4, 10);
}