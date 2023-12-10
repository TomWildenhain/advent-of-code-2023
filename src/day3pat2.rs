use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::{HashMap,HashSet};

const NON_SYMBOLS: &'static str = "1234567890.";

fn neighbor_indices(x: usize, y: usize, width: usize, height: usize) -> impl Iterator<Item=(usize, usize)> {
    let y1 = if y > 0 {y - 1} else {y};
    let x1 = if x > 0 {x - 1} else {x};
    let y2 = if y < height - 1 {y + 1} else {y};
    let x2 = if x < width - 1 {x + 1} else {x};

    return (y1..=y2).flat_map(move |cy| (x1..=x2).map(move |cx| (cx, cy)));
}

fn main() {
    let content = fs::read_to_string("./src/input3.txt").unwrap();
    let lines: Vec<&str> = content.lines().collect();

    let mut gear_to_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        let mut current_value: u32 = 0;
        let mut adjacent_gears: HashSet<(usize, usize)> = HashSet::new();


        for (x, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                current_value *= 10;
                current_value += d;

                for (x2, y2) in neighbor_indices(x, y, line.len(), lines.len()) {
                    if y != y2 || x != x2 {
                        let c2 = lines[y2].as_bytes()[x2];
                        if c2 == b'*' {
                            adjacent_gears.insert((x2, y2));
                        }
                    }
                }
            } else {
                if current_value > 0 {
                    for g in adjacent_gears.iter() {
                        match gear_to_numbers.get_mut(&g) {
                            Some(vec) => {
                                vec.push(current_value);
                            },
                            None => {
                                gear_to_numbers.insert(*g, vec![current_value]);
                            }
                        }
                    }
                }
                current_value = 0;
                adjacent_gears = HashSet::new();
            }
        }

        if current_value > 0 {
            for g in adjacent_gears.iter() {
                match gear_to_numbers.get_mut(&g) {
                    Some(vec) => {
                        vec.push(current_value);
                    },
                    None => {
                        gear_to_numbers.insert(*g, vec![current_value]);
                    }
                }
            }
        }
        current_value = 0;
        adjacent_gears = HashSet::new();
    }

    let mut sum: u32 = 0;
    for (gear, numbers) in gear_to_numbers.iter() {
        if numbers.len() == 2 {
            sum += numbers.into_iter().product::<u32>();
        }
    }

    println!("{:?}", gear_to_numbers);

    println!("{}", sum);
}