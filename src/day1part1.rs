use std::fs;

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn char_to_digit(c: char) -> u32 {
    return c.to_digit(10).unwrap();
}

fn first_digit(s: &str) -> u32 {
    for char in s.chars() {
        if is_digit(char) {
            return char_to_digit(char);
        }
    }

    panic!("No digit!")
}

fn last_digit(s: &str) -> u32 {
    for char in s.chars().rev() {
        if is_digit(char) {
            return char_to_digit(char);
        }
    }

    panic!("No digit!")
}

fn main() {
    let file_path = r"D:\Tom\Dropbox\Random\Advent of Code 2023\aoc\src\input1.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let lines = content.lines();

    let mut sum: u32 = 0;
    for line in lines {
        sum += first_digit(line) * 10 + last_digit(line);
    }

    println!("{}", sum);
}
