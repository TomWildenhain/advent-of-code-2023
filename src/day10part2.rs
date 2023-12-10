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

fn is_crossing(char: u8) -> bool {
    return char == b'-' || char == b'F' || char == b'L';
}

fn main() {
    let content = fs::read_to_string("./src/input10.txt").unwrap();
    let lines = content.lines();
    let grid: Vec<Vec<u8>> = lines.map(|line| line.as_bytes().iter().map(|c| *c).collect()).collect();
    let mut path_mask: Vec<Vec<bool>> = grid.iter().map(|row| row.iter().map(|_| false).collect()).collect();
    let height: i32 = grid.len().try_into().unwrap();
    let width: i32 = grid[0].len().try_into().unwrap();

    let (animal_x, animal_y) = 
        grid.iter().enumerate().find_map(
            |(y, row)| row.iter().enumerate().find_map(
                |(x, c)| if *c == b'S' {Some((x, y))} else {None})).unwrap();

    let initial_direction = (0, 1);

    let mut x: usize = animal_x;
    let mut y: usize = animal_y;
    let mut length: i32 = 0;
    path_mask[y][x] = true;

    let new_pos = apply_direction((x, y), initial_direction, width, height);
    (x, y) = new_pos.unwrap();
    length += 1;

    let mut prev_direction = initial_direction;
    path_mask[y][x] = true;

    loop {
        let current_char = grid[y][x];
        if current_char == b'S' {
            println!("Length: {}", length);
            println!("Dist: {}", length / 2);
            println!("Initial Direction: {:?}", initial_direction);
            break;
        }
        
        let (d1, d2) = *BYTE_TO_DIRECTIONS.get(&current_char).unwrap();

        // Wow this feels wrong, control flow in a ternary statement!!!??
        let other_d = if is_opposite_of(d1, prev_direction) {
            d2
        } else {
            d1
        };

        let new_pos = apply_direction((x, y), other_d, width, height);
        (x, y) = new_pos.unwrap();
        path_mask[y][x] = true;
        length += 1;

        prev_direction = other_d;
    }

    let mut area = 0;

    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if path_mask[y][x] {
                continue;
            }
            let mut crossings = 0;
            for r in 0..=y {
                if path_mask[r][x] && is_crossing(grid[r][x]) {
                    crossings += 1
                }
            }
            if crossings % 2 == 1 {
                area += 1;
            }
        }
    }

    println!("Area: {}", area);
}