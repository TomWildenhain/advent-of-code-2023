use std::fs;
use std::collections::HashMap;
use lazy_static::lazy_static;

type Direction = (i32, i32);

type Position = (usize, usize);

lazy_static!{
    static ref BYTE_TO_DIRECTIONS: HashMap<u8, (Direction, Direction)> = HashMap::from_iter(vec![
        (b'|', ((0, 1), (0, -1))),
        (b'-', ((1, 0), (-1, 0))),
        (b'L', ((0, -1), (1, 0))),
        (b'J', ((0, -1), (-1, 0))),
        (b'7', ((-1, 0), (0, 1))),
        (b'F', ((1, 0), (0, 1))),
    ]);
}

fn apply_direction(pos: Position, dir: Direction, width: i32, height: i32) -> Option<Position> {
    let (x, y) = pos;
    let (dx, dy) = dir;
    
    let x_i32: i32 = x.try_into().unwrap();
    let y_i32: i32 = y.try_into().unwrap();

    let new_x_i32: i32 = x_i32 + dx;
    let new_y_i32: i32 = y_i32 + dy;

    if 0 <= new_x_i32 && new_x_i32 < width && 0 <= new_y_i32 && new_y_i32 < height {
        return Some((new_x_i32.try_into().unwrap(), new_y_i32.try_into().unwrap()));
    }

    return None
}

fn is_opposite_of(d1: Direction, d2: Direction) -> bool {
    d1.0 == -d2.0 && d1.1 == -d2.1
}

fn main() {
    let content = fs::read_to_string("./src/input10.txt").unwrap();
    let lines = content.lines();
    let grid: Vec<Vec<u8>> = lines.map(|line| line.as_bytes().iter().map(|c| *c).collect()).collect();
    let height: i32 = grid.len().try_into().unwrap();
    let width: i32 = grid[0].len().try_into().unwrap();

    let (animal_x, animal_y) = 
        grid.iter().enumerate().find_map(
            |(y, row)| row.iter().enumerate().find_map(
                |(x, c)| if *c == b'S' {Some((x, y))} else {None})).unwrap();

    let directions_to_try: Vec<Direction> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];

    for initial_direction in directions_to_try {
        let mut x: usize = animal_x;
        let mut y: usize = animal_y;
        let mut length: i32 = 0;

        let new_pos = apply_direction((x, y), initial_direction, width, height);
        if new_pos.is_none() {
            continue;
        }
        (x, y) = new_pos.unwrap();
        length += 1;

        let mut prev_direction = initial_direction;

        loop {
            let current_char = grid[y][x];
            if current_char == b'.' {
                break;
            }
            else if current_char == b'S' {
                println!("Length: {}", length);
                println!("Dist: {}", length / 2);
                return;
            }
            
            let (d1, d2) = *BYTE_TO_DIRECTIONS.get(&current_char).unwrap();

            // Wow this feels wrong, control flow in a ternary statement!!!??
            let other_d = if is_opposite_of(d1, prev_direction) {
                d2
            } else if is_opposite_of(d2, prev_direction) {
                d1
            } else {
                break;
            };

            let new_pos = apply_direction((x, y), other_d, width, height);
            if new_pos.is_none() {
                continue;
            }
            (x, y) = new_pos.unwrap();
            length += 1;

            prev_direction = other_d;
        }
    }
}