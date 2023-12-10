use std::fs;
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

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

    let mut sum = 0;

    for (y, line) in lines.iter().enumerate() {
        let mut current_value = 0;
        let mut is_symbol = false;
        for (x, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                current_value *= 10;
                current_value += d;

                for (x2, y2) in neighbor_indices(x, y, line.len(), lines.len()) {
                    if y != y2 || x != x2 {
                        let c2 = lines[y2].as_bytes()[x2];
                        is_symbol = is_symbol || !NON_SYMBOLS.contains(c2 as char);
                    }
                }
            } else {
                if is_symbol {
                    sum += current_value;
                    current_value = 0;
                    is_symbol = false;
                } else {
                    current_value = 0;
                }
            }
        }

        if is_symbol {
            sum += current_value;
            current_value = 0;
            is_symbol = false;
        } else {
            current_value = 0;
        }
    }

    println!("{}", sum);
}