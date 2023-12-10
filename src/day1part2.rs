use std::fs;

fn is_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

fn char_to_digit(c: char) -> u32 {
    return c.to_digit(10).unwrap();
}

fn first_digit(s: &str) -> u32 {
    let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i, c) in s.chars().enumerate() {
        if is_digit(c) {
            return char_to_digit(c);
        }
        for (pos, d) in digits.iter().enumerate() {
            if s[i..].starts_with(d) {
                return pos.try_into().unwrap();
            }
        }
    }

    panic!("No digit!")
}

fn last_digit(s: &str) -> u32 {
    let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    //let chars = s.chars().collect::<Vec<_>>();
    for i in (0..s.len()).rev() {
        let c = s[i..i+1].chars().nth(0).unwrap();
        if is_digit(c) {
            return char_to_digit(c);
        }
        for (pos, d) in digits.iter().enumerate() {
            if s[i..].starts_with(d) {
                return pos.try_into().unwrap();
            }
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
